#![no_std]
extern crate spin;
extern crate x86_64;

use x86_64::instructions::interrupts;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T: ?Sized>(spin::Mutex<T>);
pub struct MutexGuard<'a, T: ?Sized + 'a>{
    guard: spin::MutexGuard<'a, T>,
    enable_interrupts: bool,
}

impl<T> Mutex<T> {
    pub const fn new(t: T) -> Mutex<T> {
        Mutex(spin::Mutex::new(t))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<T> {
        let enable_interrupts = interrupts::are_enabled();
        interrupts::disable();
        let guard = self.0.lock();
        MutexGuard{guard, enable_interrupts}
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        let enable_interrupts = interrupts::are_enabled();
        interrupts::disable();
        match self.0.try_lock() {
            Some(guard) => Some(MutexGuard{guard, enable_interrupts}),
            None => {
                if enable_interrupts {
                    interrupts::enable();
                }
                None
            },
        }
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {self.guard.deref()}
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {self.guard.deref_mut()}
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        if self.enable_interrupts {
            interrupts::enable();
        }
    }
}