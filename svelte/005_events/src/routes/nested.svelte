<script>
	import { createEventDispatcher } from 'svelte';
	import Nestedlv2 from './nestedlv2.svelte';

	const dispatch = createEventDispatcher();

	function sayHello() {
		dispatch('message', {
			text: 'hello',
		})
	}

	setTimeout(sayHello, 1000);
</script>

<Nestedlv2 on:message2 />

<style></style>