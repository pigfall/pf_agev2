use std::{sync::{Arc, Mutex,Condvar}, collections::VecDeque, ptr::NonNull};
use crate::events::AndroidActivityEvent;
use pf_age_third_party::log::{error, info};

pub struct AndroidCallbackMQ {
    lock: Mutex<bool>,
    cond: Condvar,
    android_activity_msg_channel: VecDeque<AndroidActivityEvent>,
    updated:bool,
}



impl AndroidCallbackMQ {
    pub fn new()->Self{
        Self{
            lock: Mutex::new(false),
            cond: Condvar::new() ,
            android_activity_msg_channel: VecDeque::new(),
            updated:true,
        }

    }
    pub fn read_msg(&mut self)->Vec<AndroidActivityEvent>{
        let guard = self.lock.lock().unwrap();
        let mut msgs = vec![];
        while !self.android_activity_msg_channel.is_empty(){
            let msg = self.android_activity_msg_channel.pop_front().expect("has checked not empty");
            msgs.push(msg);
        }
        return msgs;
    }

    pub fn push_msg_then_wait_handled(&mut self,msg :AndroidActivityEvent ){
        info!("push msg {:?}",msg);
        let mut guard = self.lock.lock().unwrap();
        self.updated = false;
        self.android_activity_msg_channel.push_back(msg);
        while !self.updated{
            info!("wating updated");
            guard =self.cond.wait(guard).unwrap();
        };
        info!("updated, unlocked");
    }

    pub fn notify_mq_msg_handled(&mut self){
        let guard = self.lock.lock().unwrap();
        self.updated = true;
        self.cond.notify_all();
    }
}

