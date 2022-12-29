<script>
	import { onMount, beforeUpdate, afterUpdate, tick } from 'svelte';

	let products = [];

	onMount(async () => {
		console.log('DEBUG: onMount');
		const res = await fetch(`https://dummyjson.com/products`);
		const text = await res.text();
		products = JSON.parse(text).products;
		textArea.select();
	});

	beforeUpdate(() => {
		console.log('DEBUG: beforeUpdate');
	});
	afterUpdate(() => {
		console.log('DEBUG: afterUpdate');
	});

	let textArea;
	let textAreaValue = `Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam sollicitudin ipsum ac diam porttitor efficitur. Nulla vitae ligula ac orci condimentum fringilla at in neque. Praesent in nisi sed mi sodales dapibus quis nec sem. Ut sodales magna ac tortor bibendum feugiat. Vestibulum maximus rutrum quam. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Etiam eget ullamcorper felis. Mauris est risus, egestas et rhoncus id, maximus in risus. Vivamus posuere justo et ligula pretium fringilla. Fusce vitae elit sapien.

	Fusce pretium sapien id placerat ullamcorper. Praesent aliquam vulputate nisi a efficitur. Nullam malesuada eu ligula a venenatis. Donec sit amet risus neque. Sed quis congue augue, nec elementum ligula. Quisque vestibulum metus id quam sagittis, eu iaculis enim maximus. Aliquam malesuada lorem ac massa maximus, quis ultrices sapien varius.

	Nulla sit amet feugiat tellus. Nam laoreet neque mattis consequat convallis. Etiam facilisis, lorem sed viverra varius, magna leo suscipit tortor, id aliquet dui erat a arcu. Mauris euismod volutpat semper. Morbi sit amet ex tellus. Sed ac sollicitudin nunc. Nam finibus mattis lacus. Ut semper, turpis vitae varius ultricies, erat neque aliquet sem, non egestas lorem diam at augue. Vivamus dignissim arcu ante, a commodo purus egestas sed.

	Etiam suscipit felis sem, in sagittis ex feugiat eu. Nunc gravida dui ac porta vehicula. Proin eu gravida orci. Phasellus at purus non quam sagittis lobortis nec ac lorem. Ut porta congue magna non posuere. Nulla consequat, tellus sit amet dapibus dapibus, elit tortor condimentum mauris, a aliquam neque lacus tincidunt sapien. Quisque sed ligula eget dolor sollicitudin suscipit. Quisque viverra suscipit erat aliquet lobortis. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Pellentesque vitae enim non turpis condimentum viverra eu eget neque. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vivamus auctor condimentum nunc, quis venenatis ligula scelerisque non.

	Vestibulum nec sollicitudin diam. Nam pretium est vitae enim interdum condimentum vitae at ante. Praesent luctus dui vitae bibendum iaculis. Cras sagittis, massa at posuere efficitur, massa massa interdum dui, sit amet dictum lorem magna eget ligula. Nullam fringilla libero sed bibendum vehicula. Proin vel ornare est. Aliquam quis risus at sapien condimentum luctus. Mauris et magna finibus libero molestie pharetra.`
	async function removeSpace() {
		textAreaValue = textAreaValue.toUpperCase();
		await tick();
		textArea.select();
	}
</script>

<ul>
	{#each products as product (product.id)}
		<li>{product.title}</li>
	{/each}
</ul>

<hr/>

<button on:click={removeSpace}>remove spaces</button>
<textarea bind:this={textArea}>{textAreaValue}</textarea>

<style>
	textarea {
		width: 700px;
		height: 200px;
	}
</style>