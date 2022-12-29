<script>
	export let num = 0;
	function increase() {
		num += 1;
	}
</script>

<button on:click={increase}>Increase {num}</button>