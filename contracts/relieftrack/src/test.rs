#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address};

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        let contract_id = env.register_contract(None, super::ReliefTrack);
        let client = super::ReliefTrackClient::new(&env, &contract_id);

        let admin = Address::random(&env);
        let recipient = Address::random(&env);

        client.init(&admin);
        client.disburse(&admin, &recipient, &100);

        let record = client.get(&0);
        assert_eq!(record.amount, 100);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized() {
        let env = Env::default();
        let contract_id = env.register_contract(None, super::ReliefTrack);
        let client = super::ReliefTrackClient::new(&env, &contract_id);

        let admin = Address::random(&env);
        let attacker = Address::random(&env);
        let recipient = Address::random(&env);

        client.init(&admin);
        client.disburse(&attacker, &recipient, &100);
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();
        let contract_id = env.register_contract(None, super::ReliefTrack);
        let client = super::ReliefTrackClient::new(&env, &contract_id);

        let admin = Address::random(&env);
        let r1 = Address::random(&env);

        client.init(&admin);
        client.disburse(&admin, &r1, &50);

        assert_eq!(client.count(), 1);
    }

    #[test]
    fn test_multiple_disbursements() {
        let env = Env::default();
        let contract_id = env.register_contract(None, super::ReliefTrack);
        let client = super::ReliefTrackClient::new(&env, &contract_id);

        let admin = Address::random(&env);
        let r1 = Address::random(&env);
        let r2 = Address::random(&env);

        client.init(&admin);
        client.disburse(&admin, &r1, &50);
        client.disburse(&admin, &r2, &75);

        assert_eq!(client.count(), 2);
    }

    #[test]
    fn test_read_record() {
        let env = Env::default();
        let contract_id = env.register_contract(None, super::ReliefTrack);
        let client = super::ReliefTrackClient::new(&env, &contract_id);

        let admin = Address::random(&env);
        let recipient = Address::random(&env);

        client.init(&admin);
        client.disburse(&admin, &recipient, &200);

        let record = client.get(&0);
        assert_eq!(record.amount, 200);
    }
}