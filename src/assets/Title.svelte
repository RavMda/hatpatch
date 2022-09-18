<div class="title">
	<h1>HatPatcher</h1>
</div>

<div id="github">
	<button id="github-button" on:click={() => window.open("https://github.com/TastyFrenchBun/hatpatcher")}>
		<img id="github-logo" src="/images/github-light.png" alt="Github Logo" />
		GitHub
	</button>
</div>

<style>
	.title {
		color: white;
		margin: auto;
		width: fit-content;
		font-size: 14px;
	}

	#github {
		position: fixed;
		width: fit-content;
		bottom: 0px;
		left: 40%;
		background-color: rgb(0, 0, 0, 0);
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