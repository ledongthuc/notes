<script>
	import Nested from './nested.svelte';
	let name = 'world';

	let num1 = 1;
	let num2 = 2;

	let checked = false;
	let checkedGroups = [];
	let checkedAvailableItems = ["1","2","3","4","5"];

	let radio = 0;

	let select;
	let selectAvailableValues = ["1", "2", "3", "4"];

	let htmlValue = 'Hello';

	let num = 0;

	let inputTextfield;
	function focusTextBox() {
		inputTextfield.focus();
	}
</script>

<input type="input" bind:value={name} />
<textarea bind:value={name}></textarea>
<p>Hello {name}</p>

<hr/>

<label>
	<input type="number" bind:value={num1} min=0 max=10>
	<input type="range" bind:value={num1} min=0 max=10>
</label>

<label>
	<input type="number" bind:value={num2} min=0 max=10>
	<input type="range" bind:value={num2} min=0 max=10>
</label>

<p>{num1} + {num2} = {num1 + num2}</p>

<hr/>

<input type="checkbox" bind:checked={checked}>
<p>{ checked ? "checked" : "unchecked" }</p>


{#each checkedAvailableItems as checkedValue}
	{checkedValue}: <input type="checkbox" bind:group={checkedGroups} name="checkedGroups" value={checkedValue} />
{/each}

<p>Checked group: {checkedGroups.join(", ")}</p>

<hr/>

1: <input type="radio" name="radio" bind:group={radio} value="1" />
2: <input type="radio" name="radio" bind:group={radio} value="2" />
3: <input type="radio" name="radio" bind:group={radio} value="3" />

<p>Radio: {radio}</p>

<hr/>

<select bind:value={select}>
	{#each selectAvailableValues as value}
		<option value={value}>{value}</option>
	{/each}
</select>

<p>Select: {select}</p>

<hr/>

<div
	contenteditable="true"
	bind:innerHTML={htmlValue}
></div>
<p>{htmlValue}</p>

<hr/>

Nested component {num}: 
<Nested bind:num={num} />

<hr/>

<input type="input" bind:this={inputTextfield} />
<button on:click={focusTextBox}>Focus textbox</button>

<style>
</style>