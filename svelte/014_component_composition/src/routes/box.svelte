<script></script>

<div class="box">
	<slot><em>no content was provided</em></slot>

	<p style="text-align: right;">
		<slot name="right"></slot>
	</p>

	<p>
		
		{#if $$slots.optionalSlot}
			<p style="color: green;"><slot name="optionalSlot"></slot></p>
		{:else}	
			<p style="color: white;">optionalSlot is not defined</p>
		{/if}
	</p>
</div>

<style>
	.box {
		background-color: red;
		padding: 10px;
	}

	.box h2 {
		color: white;
	}
</style>