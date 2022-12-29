<script>
	import { onDestroy } from 'svelte';
	import Incrementaer from './Incrementaer.svelte';
	import { count, countDouble, name } from './store.ts';
	import { count2, count2Double } from './store2.ts';
	import { customCount } from './custom_store.ts';

	let countValue;
	const unsubscribe = count.subscribe(value => {
		countValue = value;
	});
	onDestroy(unsubscribe);
</script>

<p>Count: {countValue} with double: {$countDouble}</p>
<Incrementaer countStore={count} />

<hr/>

<p>Count2: {$count2}, with double: {$count2Double}</p>
<Incrementaer countStore={count2} />

<hr/>

<p>Custom count: {$customCount}</p>
<button on:click={customCount.increment}>+</button>

<hr/>

<input type="input" bind:value={$name} />
<button on:click={() => {$name += "!"}}>Add !</button>