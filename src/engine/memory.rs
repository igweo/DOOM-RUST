use std::cell::Cell;

//TODO: Change to doubly linked list implementation of memory
const MEMORY: Vec<Option<Cell<(u8, Vec<u8>)>>> = Vec::new();
//Purge tags
//Tags < 100 are not overwritten until freed
const PU_STATIC: u8 = 1;
const PU_SOUND: u8 = 2;
const PU_MUSIC: u8 = 3;
const PU_LEVEL: u8 = 50;
const PU_LEVSPEC: u8 = 51;
// Tags >= 100 are purged when memory is needed
const PU_PURGELEVEL: u8 = 100;
const PU_CACHE: u8 = 101;
