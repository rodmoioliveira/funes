use std::{collections::HashSet, iter::FromIterator};

use log;

use crate::{api, statics};

pub fn latency_collection() {
    if statics::ENVS.latency_enable {
        let keys = statics::LATENCY_COLLECTION
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        let regex_map: Vec<_> = keys
            .iter()
            .map(|v| api::key(&v).unwrap_or("None").to_owned())
            .collect();

        if keys != regex_map {
            let keys_set: HashSet<String> = HashSet::from_iter(keys.iter().cloned());
            let regex_map_set: HashSet<String> = regex_map.iter().clone().cloned().collect();
            let diff: HashSet<_> = keys_set.difference(&regex_map_set).collect();

            log::error!(
                "FUNES_LATENCY_COLLECTION keys:\n {:#?}\n\nDiffer from \
                 regex_map:\n{:#?}\n\nBecause FUNES_API_REGEX: {} doesn't match all keys.\nThe \
                 offenders are the following:\n\n{:?}\n",
                keys_set,
                regex_map_set,
                statics::ENVS.api_regex,
                diff,
            );
            panic!();
        }
    }
}
