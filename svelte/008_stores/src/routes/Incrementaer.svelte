<script>
	export let countStore;

	function increment() {
		countStore.update(n => n + 1);
	}
</script>

<button on:click={increment}>+</button>