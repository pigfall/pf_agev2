use pf_age_ndk::{ANativeWindow, AInputQueue,AInputQueue_getEvent,AInputQueue_finishEvent};
use pf_age_third_party::log::info;
use std::{sync::{Arc, Mutex,Condvar}, collections::VecDeque};
use crate::{events::{Event,AndroidActivityEvent}, render};
use crate::render::{RenderTrait,GLRender};
use pf_age_third_party::EventChannel;

pub struct GameLooper<Render:RenderTrait>{
    lock: Mutex<bool>,
    cond: Condvar,
    updated: bool,
    android_activity_msg_channel: VecDeque<AndroidActivityEvent>,
    render: Render,
    input_queue:*mut AInputQueue,
    game_ev_channel: *mut EventChannel<Event>,
}

impl<Render:RenderTrait> GameLooper<Render> {
    pub fn new(render:Render)->GameLooper<Render>{
        return GameLooper { 
            lock: Mutex::new(false),
             cond: Condvar::new() ,
             updated:false,
             android_activity_msg_channel:VecDeque::new(),
             render: render,
             input_queue:std::ptr::null_mut(),
             game_ev_channel: Box::into_raw(Box::new(EventChannel::new())),
            }
    }

    // android_loop_run
    pub fn loop_run(&mut self){
        loop{
            self.pre_handle_android_activitiy_evs();
            // poll input events;
            self.poll_input_events();
            //app.one_frame()
        }
    }

    fn poll_input_events(&mut self){
        if self.input_queue.is_null(){
            return
        }
        loop{
            let mut android_input_ev = std::ptr::null_mut();
            unsafe{
                if AInputQueue_getEvent(self.input_queue, &mut android_input_ev) <0{
                    break
                };
                AInputQueue_finishEvent(self.input_queue,android_input_ev,1);
                // TODO write to game ev channel
                // self.game_ev_channel.single_write()
            }
        }
    }

    fn pre_handle_android_activitiy_evs(&mut self){
        let guard = self.lock.lock().unwrap();
        while !self.android_activity_msg_channel.is_empty(){
            let msg = self.android_activity_msg_channel.pop_front().expect("has checked not empty");
            match msg{
                AndroidActivityEvent::WindowCreate(window_raw) =>{
                    info!("rcv msg window created");
                    self.render.on_window_create(window_raw as _ );
                },
                AndroidActivityEvent::WindowDestroy=>{
                    info!("rcv msg window destroy");
                    self.render.on_window_destroy(std::ptr::null_mut());
                },
                AndroidActivityEvent::InputQueueCreated(queue)=>{
                    info!("rcv msg input queue created");
                    self.input_queue = queue;
                },
                AndroidActivityEvent::onInputQueueDestroyed=>{
                    info!("rcv msg input destroy");
                    self.input_queue = std::ptr::null_mut();
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
