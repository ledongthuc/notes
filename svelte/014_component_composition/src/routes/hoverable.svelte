<script>
	let hovering = false;
</script>

<div on:mouseenter={() => hovering = true} on:mouseleave={() => hovering = false}>
	<slot {hovering} />
</div>