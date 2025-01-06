use crate::prelude::*;
use crate::tricks_config::Trick;
use std::collections::BTreeMap;

pub struct AllTricksStatus(BTreeMap<TrickID, TrickStatus>);

#[derive(Debug, Clone)]
pub struct TrickStatus {
    pub trick: Trick,
    pub actions: Vec<ActionDisplayStatus>,
}

#[derive(Debug, Clone)]
pub struct ActionDisplayStatus {
    pub action_id: SpecificActionID,
    pub is_available: bool,
    pub is_ongoing: bool,
}

impl AllTricksStatus {
    pub fn new(providers: Vec<DynTrickProvider>) -> Self {
        let mut trick_map = BTreeMap::new();

        for provider in providers {
            let trick = provider.get_trick();
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
            let trick_status = TrickStatus {
                trick: trick.clone(),
                actions,
            };
            trick_map.insert(trick_id, trick_status);
        }

        AllTricksStatus(trick_map)
    }

    fn get_tricks_btree(&self) -> &BTreeMap<TrickID, TrickStatus> {
        &self.0
    }

    fn get_all_tricks(&self) -> Vec<(TrickID, TrickStatus)> {
        self.0.clone().into_iter().collect()
    }

    fn get_all_tricks_in_category<'a>(&self, category_id: &String) -> Vec<(TrickID, TrickStatus)> {
        self.get_all_tricks()
            .into_iter()
            .filter(move |t| t.1.trick.categories.contains(category_id))
            .collect()
    }

    pub fn get_full_map_for_categories(
        &self,
        categories: Vec<CategoryID>,
    ) -> Vec<(CategoryID, Vec<(TrickID, TrickStatus)>)> {
        let mut category_to_tricks = vec![("all".into(), self.get_all_tricks())];
        for category in categories {
            let tricks: Vec<(String, TrickStatus)> = self.get_all_tricks_in_category(&category);
            category_to_tricks.push((category.clone(), tricks));
        }
        category_to_tricks
    }
}
