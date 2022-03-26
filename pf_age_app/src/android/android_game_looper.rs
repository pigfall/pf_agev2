use pf_age_ndk::{ANativeWindow, AInputQueue,AInputQueue_getEvent,AInputQueue_finishEvent,InputEvent};
use pf_age_third_party::log::info;
use std::{sync::{Arc, Mutex,Condvar}, collections::VecDeque, ptr::NonNull};
use crate::{events::{Event,AndroidActivityEvent, AndroidInputWrapper, ANativeWindowWrapper, AInputQueueWrapper}, render};
use crate::render::{RenderTrait,GLRender};
use pf_age_third_party::{EventChannel,ReaderId};

pub struct GameLooper<Render:RenderTrait>{
    lock: Mutex<bool>,
    cond: Condvar,
    updated: bool,
    android_activity_msg_channel: VecDeque<AndroidActivityEvent>,
    render: Render,
    input_queue:*mut AInputQueue,
    game_ev_channel: *mut EventChannel<Event>,
    game_ev_reader: ReaderId<Event>,
}

impl<Render:RenderTrait> GameLooper<Render> {
    pub fn new(render:Render)->GameLooper<Render>{
        let mut game_ev_channel = Box::new(EventChannel::with_capacity(100));
        let mut game_ev_reader = game_ev_channel.register_reader();
        return GameLooper { 
            lock: Mutex::new(false),
             cond: Condvar::new() ,
             updated:false,
             android_activity_msg_channel:VecDeque::new(),
             render: render,
             input_queue:std::ptr::null_mut(),
             game_ev_channel: Box::into_raw(game_ev_channel),
             game_ev_reader:game_ev_reader,
            }
    }

    // android_loop_run
    pub fn loop_run(&mut self){
        loop{
            self.pre_handle_android_activitiy_evs();
            // poll input events;
             self.poll_input_events();
            //app.one_frame()
            unsafe{
                for ev in (*self.game_ev_channel).read(&mut self.game_ev_reader){
                    info!("read from game ev channel {:?}",ev);
                }
            }
        }
    }

    fn poll_input_events(&mut self){
        if self.input_queue.is_null(){
            return
        }
        loop{
            let mut android_input_ev_raw = std::ptr::null_mut();
            unsafe{
                if AInputQueue_getEvent(self.input_queue, &mut android_input_ev_raw) <0{
                    break
                };
                let android_input_ev  = InputEvent::from_ptr(NonNull::new(android_input_ev_raw).unwrap());
                AInputQueue_finishEvent(self.input_queue,android_input_ev_raw,1);
                (*self.game_ev_channel).single_write(Event::AndroidInputEvent(AndroidInputWrapper{ev:android_input_ev}))
            }
        }
    }

    fn pre_handle_android_activitiy_evs(&mut self){
        let guard = self.lock.lock().unwrap();
        while !self.android_activity_msg_channel.is_empty(){
            let msg = self.android_activity_msg_channel.pop_front().expect("has checked not empty");
            match msg{
                AndroidActivityEvent::WindowCreate(wrapper) =>{
                    info!("rcv msg window created");
                    self.render.on_window_create(wrapper.window as _);
                },
                AndroidActivityEvent::WindowDestroy=>{
                    info!("rcv msg window destroy");
                    self.render.on_window_destroy(std::ptr::null_mut());
                },
                AndroidActivityEvent::InputQueueCreated(queue)=>{
                    info!("rcv msg input queue created");
                    self.input_queue = queue.queue;
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
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::WindowCreate(ANativeWindowWrapper{window:window_raw}));
    }

    pub fn destroy_window(&mut self,window_raw: *mut ANativeWindow){
        info!("destroy window");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::WindowDestroy);
    }

    pub fn set_input_queue(&mut self,queue:*mut AInputQueue){
        info!("set input queeu");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::InputQueueCreated(AInputQueueWrapper{queue:queue}));
    }
    pub fn destroy_input_queue(&mut self,queue:*mut AInputQueue){
        info!("destroy input queue");
        self.get_lock_to_notify_msg_and_wait_handled(AndroidActivityEvent::onInputQueueDestroyed);
    }
}
