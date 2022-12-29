<script>
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function sayHello() {
		dispatch('message2', {
			text: 'hello2',
		})
	}

	setTimeout(sayHello, 5000);
</script>

<style></style>