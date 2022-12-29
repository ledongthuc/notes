<script>
	import Nested from './nested.svelte';
	import Nested2 from './nested2.svelte';
	let nested2Props = {
		firstNumber: 1,
		secondNumber: 2,
	};
</script>

<Nested answer="1" />
<Nested2 {...nested2Props} />

<style></style>