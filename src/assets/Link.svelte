<div id="github">
	<button id="github-button" on:click={() => window.open("https://github.com/TastyFrenchBun/hatpatcher")}>
		<img id="github-logo" src="/images/github-light.png" alt="Github Logo" />
		GitHub
	</button>
</div>

<style>
	#github {
		position: fixed;
		width: fit-content;
		bottom: 10px;
		background-color: rgb(0, 0, 0, 0);
		left: 0;
		right: 0;
		margin: auto;
	}

	#github-button, #github-logo {
		transition: var(--anim-speed);
	}

	#github-button:hover {
		color: black;
	}

	#github-button:hover #github-logo {
		filter: invert(1);
	}

	#github-button {
		background-color: rgb(0, 0, 0, 0);
		border: 0;
		color: white;
		padding: 0;
	}

	#github-logo {
		width: 14px;
	}
</style>