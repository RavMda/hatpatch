import { writable } from 'svelte/store';

export const 
	readyToPatch = writable(false),
	gamePath = writable("");