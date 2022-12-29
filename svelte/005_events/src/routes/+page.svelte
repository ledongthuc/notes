<script>
	import Nested from './nested.svelte';

	let mouse = { x:0, y:0 };

	function mouseMove(event) {
		mouse.x = event.clientX;
		mouse.y = event.clientY;
	}

	let isClicked = false;
	function clickMe() {
		isClicked = !isClicked;
	}

	function nestedChildMessage(e) {
		alert(e.detail.text);
	}
</script>

<div on:mousemove={mouseMove} style="background-color: red;">
	The mouse position is {mouse.x} x {mouse.y}
</div>
<div on:mousemove={e => mouse = { x: event.clientX, y: event.clientY }} style="top: 50%; background-color: blue;">
	The mouse2 position is {mouse.x} x {mouse.y}
	<p><button on:click|once={clickMe}>{isClicked ? "clicked" : "click me"}</button></p>

	<Nested on:message={nestedChildMessage} on:message2={nestedChildMessage} />
</div>

<style>
	div {
		width: 100%;
		height: 50%;
		position: absolute;
	}
</style>