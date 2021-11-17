//Importações
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

//Alocação de memória
near_sdk::setup_alloc!();

// 1. Main Struct
/*
 Essas palavras com # são Uma tag declarativa que é usada para
transmitir informações para o tempo de execução sobre os comportamentos
de vários elementos como classes, métodos, estruturas, enumeradores,
assemblies, etc.

A primeira parte do código tem uma estrutura(que serve para a parte de
armazenamento do contrato), e depois temos a parte de implementação.
*/

#[near_bindgen]//Faz ser compativel com a near
#[derive(BorshDeserialize, BorshSerialize)]//auxilia na serialização e desserialização dos dados para enviá-los ou recuperá-los do NEAR.
pub struct KeyValue {
    pairs: UnorderedMap<String, String>, //UnorderedMap é uma estrutura de dados que utiliza o armazenamento de trie blockchain subjacente com mais eficiência
}
// 2. Default Implementation
impl Default for KeyValue {
    fn default() -> Self { //Essa func retorna keyValue como um unordered map
        Self {
            pairs: UnorderedMap::new(b"r".to_vec())
        }
    }
}


// 3. Core Logic // Adicionando os metodos para o programa
#[near_bindgen]
impl KeyValue {
    pub fn create_update(&mut self, k: String, v: String) {
        env::log(b"created or updated");//log
        self.pairs.insert(&k, &v);//criando key-value
    }

    pub fn read(&self, k: String) -> Option<String> {
        env::log(b"read");
        return self.pairs.get(&k);//pegar o key
    }

    pub fn delete(&mut self, k: String) {
        env::log(b"delete");
        self.pairs.remove(&k);//remover o key
    }
}


// 4. Tests // Parte de teste unitários 
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    //Criando uma virtual machine para os testes e mockando algumas coisas
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // Test 1
    #[test]
        fn create_read_pair() {
            let context = get_context(vec![], false);//contexto 
            testing_env!(context);//passando contexto para o ambiente de teste
            let mut contract = KeyValue::default();//contem o contrato ja criado
            contract.create_update("first_key".to_string(), "hello".to_string());
            assert_eq!(
                "hello".to_string(),
                contract.read("first_key".to_string()).unwrap()//unwrap pois n retorna string
            );
        }
    // Test 2
    #[test]
        fn read_nonexistent_pair() {
            let context = get_context(vec![], true);
            testing_env!(context);
            let contract = KeyValue::default();
            assert_eq!(None, contract.read("first_key".to_string()));
        }
}