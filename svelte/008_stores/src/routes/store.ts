import { writable, readable } from 'svelte/store';

export const count = writable(0);
export const countDouble = readable(0, function(set) {
	count.subscribe(value => {
		set(value * 2);
	})
});

export const name = writable("Thuc");