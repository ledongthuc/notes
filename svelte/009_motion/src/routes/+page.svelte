<script>
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { spring } from 'svelte/motion';

	let currentProgress = 0;
	const progress = tweened(0, {
		duration: 1000,
		easing: cubicOut,
	})

	function toggle() {
		if (currentProgress === 0) {
			progress.set(1);
			currentProgress = 1;
		} else {
			progress.set(0);
			currentProgress = 0;
		}
	}

	let coords = spring({ x: 50, y: 50 });
	function move(e) {
		coords.set({
			x: e.clientX,
			y: e.clientY,
		})
	}
</script>

<div class="cover" on:mousemove="{move}">
	<progress value={$progress}></progress>
	<button on:click={toggle}>Go</button>


	<div class="point" style="left: {$coords.x}px; top: {$coords.y}px;">
	</div>
</div>
	
<style>
	progress {
		display: block;
		width: 100%;
	}
	.cover {
		width: 100%;
		height: 100%;
		background-color: blue;
		position: absolute;
		top: 0px;
		left: 0px;
	}
	.point {
		position: absolute;
		width: 100px;
		height: 100px;
		background-color: red;
	}
</style>