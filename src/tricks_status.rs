use crate::prelude::*;
use crate::tricks_config::Trick;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;

pub struct AllTricksStatus(BTreeMap<TrickID, TrickStatus>);

pub struct TrickStatus {
    pub trick: Trick,
    pub actions: Vec<ActionDisplayStatus>,
}

pub struct ActionDisplayStatus {
    pub action_id: SpecificActionID,
    pub is_available: bool,
    pub is_ongoing: bool,
}

impl AllTricksStatus {
    pub fn new(executor: &Executor, logger: &LoggerRc) -> Self {
        let mut trick_map = BTreeMap::new();
        let tricks_and_providers = executor.get_all_providers(logger);

        for (trick, provider) in tricks_and_providers {
            let is_installing = provider.is_installing();
            let is_running = provider.is_running();
            let available_actions = provider.get_available_actions();
            let all_actions = provider.get_all_actions();

            let mut actions = vec![];
            for action_id in all_actions {
                let is_ongoing = match action_id {
                    SpecificActionID::Install => is_installing,
                    SpecificActionID::Run => is_running,
                    _ => false,
                };
                let is_available = available_actions.contains(&action_id);
                actions.push(ActionDisplayStatus {
                    action_id,
                    is_available,
                    is_ongoing,
                });
            }

            let trick_id = trick.id.clone();
            let trick_status = TrickStatus { trick, actions };
            trick_map.insert(trick_id, trick_status);
        }

        AllTricksStatus(trick_map)
    }

    fn get_tricks_btree(&self) -> &BTreeMap<TrickID, TrickStatus> {
        &self.0
    }

    fn get_all_tricks<'a>(&'a self) -> impl Iterator<Item = (&'a TrickID, &'a TrickStatus)> + 'a {
        self.0.iter()
    }

//    fn get_all_tricks_in_category<'a>(
//        &'a self,
//        category_id: &'a str,
//    ) -> impl Iterator<Item = (&'a TrickID, &'a TrickStatus)> + 'a {
//        self.get_all_tricks().filter(move |t| t.1.trick.categories.contains(category_id))
//    }
}
