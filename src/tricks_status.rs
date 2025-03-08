use std::rc::Rc;
use crate::prelude::*;
use crate::tricks_config::Trick;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TrickStatus {
    pub trick: Rc<Trick>,
    // NOTE: this should probably be a BTreeMap for perf reasons,
    //       but we're dealing with so few elements here that
    //       it isn't worth the time to rewrite everything
    //       and define Ord for SpecificActionID
    pub actions: Vec<ActionDisplayStatus>,
}

#[derive(Debug, Clone, Default)]
pub struct ActionDisplayStatus {
    pub trick: Rc<Trick>,
    pub action_id: SpecificActionID,
    pub is_available: bool,
    pub is_ongoing: bool,
    pub is_completed: bool,
}

pub struct AllTricksStatus(BTreeMap<TrickID, TrickStatus>);

impl AllTricksStatus {
    #[must_use]
    pub fn new(providers: Vec<DynTrickProvider>) -> Self {
        let mut trick_map = BTreeMap::new();

        for provider in providers {
            let trick = Rc::new(provider.get_trick().clone());
            let is_installing = provider.is_installing();
            let is_running = provider.is_running();
            let is_added_to_steam = provider.is_added_to_steam();
            let available_actions = provider.get_available_actions();
            let all_actions = provider.get_all_actions();

            let mut actions = vec![];
            for action_id in all_actions {
                let is_ongoing = match action_id {
                    SpecificActionID::Install => is_installing,
                    SpecificActionID::Run => is_running,
                    _ => false,
                };
                let is_completed = match action_id {
                    SpecificActionID::AddToSteam => is_added_to_steam,
                    _ => false,
                };
                let is_available = available_actions.contains(&action_id);
                actions.push(ActionDisplayStatus {
                    trick: trick.clone(),
                    action_id,
                    is_available,
                    is_ongoing,
                    is_completed,
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

    pub fn get(&self, trick_id: &TrickID) -> Option<&TrickStatus> {
        self.0.get(trick_id)
    }

    fn get_all_tricks(&self) -> Vec<(TrickID, TrickStatus)> {
        self.0.clone().into_iter().collect()
    }

    fn get_all_tricks_in_category(&self, category_id: &String) -> Vec<(TrickID, TrickStatus)> {
        self.get_all_tricks()
            .into_iter()
            .filter(move |t| t.1.trick.categories.contains(category_id))
            .collect()
    }

    #[must_use]
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
