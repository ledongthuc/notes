<script>
	import Nested from './Nested.svelte';

	let name = 'Thuc';
	let img_src = 'https://i.imgflip.com/75ri0w.jpg';
	let img_alt = 'image atribute';
	let html_string = '<b>Bold</b> <i>Italic</i>';
</script>

<h1>Hello {name}!</h1>

<img src={img_src} alt={img_alt}>

<Nested />

<p>{@html html_string}</p>

<style>
	h1 {
		color: purple;
	}
</style>