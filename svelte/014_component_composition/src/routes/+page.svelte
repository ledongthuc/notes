<script>
	import Box from './box.svelte';
	import Hoverable from './hoverable.svelte';
</script>

<Box>
	<h2>Hello</h2>
	<p>This is a box. It contains anything</p>
	<span slot="right">
		It should be on right
	</span>
	<span slot="optionalSlot">Hello</span>
</Box>
<br/>
<Box>
</Box>

<hr />

<Hoverable let:hovering={active}>
	{#if active}
		Go out
	{:else}
		Come here
	{/if}
</Hoverable>

<style>
	h2 {
		color: yellow;
	}
</style>