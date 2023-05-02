    
pub mod event;
pub mod cila {    

    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    #[derive(scale::Encode, scale::Decode)]
    #[derive(Clone)]
    pub struct OmnichainEvent {
        aggregate_id: String,
        command_id: String,
        operation_id: String,
        original: Vec<u8>,
    }

    pub struct EventStore {
        streams: Mapping<String, Vec<OmnichainEvent>>
    }

    pub struct  AggreagteState {

    }

    pub struct Aggregate {
        id: String
    }


    pub struct  OmnichainCommand {

    }

    impl Aggregate {

        pub fn new(id: String) -> Self {
            Self {
                id: id
                }
        }
        

        fn handle_command(&mut self, cmd: OmnichainCommand) {
            // To be implemented by derived contracts
        }
    }

    impl EventStore {
        pub fn new() -> Self {
            Self {
                streams: Mapping::new(),
            }
        }
    
        pub fn append(&mut self, aggregate_id: String, evnt: OmnichainEvent) -> u32 {
            if (!self.streams.contains(&aggregate_id)) {
                self.streams.insert(&aggregate_id, &Vec::<OmnichainEvent>::new());
            }
            let mut stream = self.streams.get(&aggregate_id).unwrap();
            let index = stream.len();
            stream.insert(index, evnt);
            return index as u32;
        }
    
        pub fn get_events(&self, aggregate_id: String) -> Vec<OmnichainEvent> {
            match self.streams.get(&aggregate_id) {
                Some(events) => events.clone(),
                None => Vec::new(),
            }
        }
    
        pub fn remove(&mut self, aggregate_id: String, idx: u32) {
            let mut stream = self.streams.get(&aggregate_id).expect("Stream not found");
            assert!(idx < stream.len() as u32, "Index out of bounds");
    
            for i in idx as usize..stream.len() - 1 {
                stream[i] = stream[i + 1].clone();
            }
            stream.pop();
        }

        pub fn remove_range(&mut self, aggregate_id: String, start_idx: u32, count: u32) {
            let mut stream = self.streams.get(&aggregate_id).expect("Stream not found");
            assert!(start_idx < stream.len() as u32, "Index out of bounds");
    
            let end_idx = start_idx + count;
            assert!(end_idx <= stream.len() as u32, "Index out of bounds");
    
            stream.splice(start_idx as usize..end_idx as usize, std::iter::empty());
        }
    
        pub fn clean(&mut self, aggregate_id: String) {
            self.streams.remove(&aggregate_id);
        }
    }
}