import { writable, derived } from 'svelte/store';

export const count2 = writable(0);

export const count2Double = derived(count2, $count2 => $count2 * 2);