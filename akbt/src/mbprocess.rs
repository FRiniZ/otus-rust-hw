#![allow(dead_code)]
use std::{collections::HashMap, sync::Arc};

use indicatif::{HumanCount, MultiProgress, ProgressBar, ProgressFinish, ProgressStyle};
use std::sync::Mutex;

type PartitionID = i32;

struct PartitionItem {
    lastoffset: i64,
    lastpublished: i64,
    finished: bool,
}

type HashMapPartitions = HashMap<PartitionID, PartitionItem>;

const PB_PROCESS: &str = "{spinner:.green} [{elapsed_precise}]: [{bar:.green/red}] ETA:{eta}";
const PB_HEADER_B1: &str = "Topic       : {msg}";
const PB_HEADER_B2: &str = "Partitions  : {len} Done: {pos}";
const PB_HEADER_B3: &str = "Records     : {human_pos} Total: {human_len}";
const PB_HEADER_R1: &str = "Topic       : {msg}";
const PB_HEADER_R2: &str = "Archive     : {msg}";
const PB_HEADER_R3: &str = "Read bytes  : {human_pos} Total: {human_len}";
const PB_FINISH: &str = "{spinner:.green} {msg:>12} {bar:.green/red} done {elapsed_precise}";

enum Action {
    Backup,
    Restore,
}

pub struct MProgressBars {
    hidden: bool,
    action: Action,
    mb: MultiProgress,
    hashmap: HashMapPartitions,
    header1: ProgressBar,
    header2: ProgressBar,
    header3: ProgressBar,
    progressbar: ProgressBar,
}

impl MProgressBars {
    pub fn backup(topic: String, count: usize, hidden: bool) -> Arc<Mutex<Self>> {
        let mb = MultiProgress::new();
        let hashmap: HashMapPartitions = HashMap::with_capacity(count);

        let header1 = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            let pb = mb.add(
                ProgressBar::new(0).with_style(ProgressStyle::with_template(PB_HEADER_B1).unwrap()),
            );

            pb.set_message(topic);
            pb.finish();
            pb
        };

        let header2 = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            let pb = mb.add(
                ProgressBar::new(0).with_style(ProgressStyle::with_template(PB_HEADER_B2).unwrap()),
            );
            pb.set_length(count as u64);
            pb
        };

        let header3 = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            mb.add(
                ProgressBar::new(0).with_style(ProgressStyle::with_template(PB_HEADER_B3).unwrap()),
            )
        };

        let progressbar = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            let pb = mb.add(ProgressBar::new(0).with_finish(ProgressFinish::AndLeave));
            pb.set_style(
                ProgressStyle::with_template(PB_PROCESS)
                    .unwrap()
                    .progress_chars("=>-"),
            );
            pb
        };

        Arc::new(Mutex::new(Self {
            mb,
            action: Action::Backup,
            hashmap,
            hidden,
            header1,
            header2,
            header3,
            progressbar,
        }))
    }

    pub fn restore(topic: String, file: String, hidden: bool) -> Arc<Mutex<Self>> {
        let mb = MultiProgress::new();
        let hashmap: HashMapPartitions = HashMap::with_capacity(1);

        let header1 = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            let pb = mb.add(
                ProgressBar::new(0).with_style(ProgressStyle::with_template(PB_HEADER_R1).unwrap()),
            );

            pb.set_message(topic);
            pb.finish();
            pb
        };

        let header2 = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            let pb = mb.add(
                ProgressBar::new(0).with_style(ProgressStyle::with_template(PB_HEADER_R2).unwrap()),
            );
            pb.set_message(file);
            pb
        };

        let header3 = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            mb.add(
                ProgressBar::new(0).with_style(ProgressStyle::with_template(PB_HEADER_R3).unwrap()),
            )
        };

        let progressbar = if hidden {
            mb.add(ProgressBar::hidden())
        } else {
            let pb = mb.add(ProgressBar::new(0).with_finish(ProgressFinish::AndLeave));
            pb.set_style(
                ProgressStyle::with_template(PB_PROCESS)
                    .unwrap()
                    .progress_chars("=>-"),
            );
            pb
        };

        Arc::new(Mutex::new(Self {
            mb,
            action: Action::Restore,
            hashmap,
            hidden,
            header1,
            header2,
            header3,
            progressbar,
        }))
    }

    pub fn add_pb(&mut self, id: i32, min: i64, max: i64) {
        match self.action {
            Action::Backup => (),
            Action::Restore => {
                if self.hashmap.len() >= 1 {
                    panic!("Can't be few progress bar for restore process");
                }
            }
        }

        let part_item = PartitionItem {
            lastoffset: min,
            lastpublished: min,
            finished: false,
        };

        match self.hashmap.insert(id, part_item) {
            Some(_) => panic!("Can't insert twice PartitionID:{}", id),
            None => (),
        }
        if !self.hidden {
            self.progressbar
                .set_length(self.progressbar.length().unwrap() + max as u64);
        }
    }

    pub fn update(&mut self, id: PartitionID, pos: i64) {
        self.hashmap.get_mut(&id).unwrap().lastoffset = pos;
    }

    pub fn finish_partition(&mut self, id: PartitionID) {
        if !self.hidden {
            self.hashmap.get_mut(&id).unwrap().finished = true;
        }
    }

    pub fn finish(&mut self) {
        if !self.hidden {
            self.header1.finish();
            self.header2.finish();
            self.header3.finish();
            self.progressbar.finish();
        }
    }

    pub fn tick(&mut self) {
        let mut diff = 0;
        let mut finished = 0;
        for (_, v) in self.hashmap.iter_mut() {
            diff += v.lastoffset - v.lastpublished;
            v.lastpublished = v.lastoffset;
            if v.finished {
                finished += 1;
            }
        }
        self.header2.set_position(finished);
        self.header3.set_length(self.progressbar.length().unwrap());
        self.header3.set_position(self.progressbar.position());
        self.header3.set_message(format!(
            "{} Total: {}",
            self.progressbar.position(),
            HumanCount(self.progressbar.length().unwrap())
        ));
        self.progressbar.inc(diff as u64);
        self.header3.tick();
    }
}
