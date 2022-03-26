use pf_age_ndk::{ANativeWindow, AInputQueue};
use pf_age_third_party::log::info;
use std::{sync::{Arc, Mutex,Condvar}, collections::VecDeque};
use crate::events::{Event,AndroidActivityEvent};

pub struct GameLooper{
    lock: Mutex<bool>,
    cond: Condvar,
    updated: bool,
    android_activity_msg_channel: VecDeque<AndroidActivityEvent>,
}

impl GameLooper {
    pub fn new()->GameLooper{
        return GameLooper { lock: Mutex::new(false), cond: Condvar::new() ,updated:false,android_activity_msg_channel:VecDeque::new()}
    }
    pub fn loop_run(&mut self){
        loop{
            self.pre_handle_android_activitiy_evs();
            //app.one_frame()
        }
    }
    fn pre_handle_android_activitiy_evs(&mut self){
        let guard = self.lock.lock().unwrap();
        while !self.android_activity_msg_channel.is_empty(){
            let msg = self.android_activity_msg_channel.pop_front().expect("has checked not empty");
            match msg{
                AndroidActivityEvent::WindowCreate(window_raw) =>{
                    info!("rcv msg window created");
                },
                AndroidActivityEvent::WindowDestroy=>{
                    info!("rcv msg window destroy");
                },
                AndroidActivityEvent::InputQueueCreated(queue)=>{
                    info!("rcv msg input queue created");
                },
                AndroidActivityEvent::onInputQueueDestroyed=>{
                    info!("rcv msg input destroy");
                },
            };
            info!("self.updated=true");
            self.updated = true;
            self.cond.notify_all();
        }
    }


    fn get_lock_to_notify_msg_and_wait_handled(&mut self,msg: AndroidActivityEvent){
        info!("send msg {:?}",msg);
        let mut guard = self.lock.lock().unwrap();
        self.updated = false;
        self.android_activity_msg_channel.push_back(msg);
        while !self.updated{
            info!("wating updated");
            guard =self.cond.wait(guard).unwrap();
        };
        info!("updated, unlocked");
    }

    pub fn set_window_raw(&mut self,window_raw: *mut ANativeWindow){
        info!("set window raw");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::WindowCreate(window_raw));
    }

    pub fn destroy_window(&mut self,window_raw: *mut ANativeWindow){
        info!("destroy window");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::WindowDestroy);
    }

    pub fn set_input_queue(&mut self,queue:*mut AInputQueue){
        info!("set input queeu");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::InputQueueCreated(queue));
    }
    pub fn destroy_input_queue(&mut self,queue:*mut AInputQueue){
        info!("destroy input queue");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::onInputQueueDestroyed);
    }
}
