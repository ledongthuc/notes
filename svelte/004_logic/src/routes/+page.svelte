<script>
	let user = {
		loggedIn: false,
	};
	function toggle() {
		user.loggedIn = !user.loggedIn;
		x++;
	}

	let x = 0;

	let cats = [
		{ id: 'J---aiyznGQ', name: 'Keyboard Cat' },
		{ id: 'z_AbfPXTKms', name: 'Maru' },
		{ id: 'OUtn3pvWmpg', name: 'Henri The Existential Cat' }
	];
	const emojis = {
        'J---aiyznGQ': "🍎",
        'z_AbfPXTKms': "🍌",
        'OUtn3pvWmpg': "🥕",
	};
	const emojis2 = ["🍎","🍌","🥕"]
	function removeFirst() {
		cats = cats.slice(1);
	}


	async function loadProducts() {
		const res = await fetch(`https://dummyjson.com/products`);
		const text = await res.text();

		if (res.ok) {
			return JSON.parse(text).products;
		} else {
			throw new Error(text);
		}
	}

	let promise = loadProducts();

	function handleLoadProductClick() {
		promise = loadProducts();
	}
</script>

{#if user.loggedIn}
	<button on:click={toggle}>Log out</button>
{:else}
	<button on:click={toggle}>Log in</button>
{/if}

{#if x === 0}
	x = 0
{:else if x > 10}
	x > 10
{:else}
	x between 0 and 10
{/if}


<hr/>

<ul>
{#each cats as cat (cat.id)}
	<li>ID: {cat.id}, name: {cat.name} - {emojis[cat.id]}</li>
{/each}
</ul>

<button on:click={removeFirst}>Remove first thing</button>

<ul>
{#each cats as cat, i}
	<li>Index: {i} - ID: {cat.id}, name: {cat.name} - {emojis2[i]}</li>
{/each}
</ul>

<hr/>

<button on:click={handleLoadProductClick}>Load products</button>

{#await promise}
	<p>Loading</p>
{:then products}
	<ul>
		{#each products as product (product.id)}
			<li>{product.title}</li>
		{/each}
	</ul>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

<style></style>