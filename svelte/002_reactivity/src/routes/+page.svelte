<script>
	let count = 0;
	$: doubled = count * 2;
	$: if (count > 0 && count % 10 == 0) {
		alert('count: ' + count);
	}
	function incrementCount() {
		count += 1;
	}

	let numbers = [1,2,3,4,5];
	$: sum = numbers.reduce((t, n) => t + n, 0);
	function addNumberToSum() {
		numbers = [...numbers, numbers.length + 1];
	}

	let obj = {obj_counter: 1};
	function incrementObjectCounter() {
		obj.obj_counter += 1;
	}
</script>

<button on:click={incrementCount}>Clicked {count} {count === 1 ? 'time' : 'times'}</button>
<p>{count} doubled is {doubled}</p>

<p>{numbers.join(' + ')} = {sum}</p>
<button on:click={addNumberToSum}>Add number to sum</button>

<br/>
<button on:click={incrementObjectCounter}>[Obj.Counter] Clicked {obj.obj_counter} {obj.obj_counter === 1 ? 'time' : 'times'}</button>

<style>
	
</style>