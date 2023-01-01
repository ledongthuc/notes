<script>
	export let data;
</script>

<div class="header">
	Top header
	<ul>
	{#each data.items as item}
		<li>{item}</li>
	{/each}
	</ul>
</div>

<slot></slot>

<div  class="footer">Footer</div>

<style>
	.header {
		width: 100%;
		background-color: red;
		color: white;
	}
	.footer {
		width: 100%;
		background-color: blue;
		color: white;
	}
</style>