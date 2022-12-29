export function clickOutside(node, message) {
	const handleClick = (event) => {
		if (!node.contains(event.target)) {
			node.dispatchEvent(new CustomEvent("outclick"));
			console.log('DEBUG: ', message);
		}
	};

	document.addEventListener("click", handleClick, true);

	return {
		update(newMessage) {
			message = newMessage;
		},
		destroy() {
			document.removeEventListener("click", handleClick, true);
		}
	};
}
