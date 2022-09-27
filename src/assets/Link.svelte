<script lang="ts">
	/*
	 * Font Awesome Icons
	 */
	import Fa from "svelte-fa";
	import { faDiscord, faTelegram } from "@fortawesome/free-brands-svg-icons"
</script>

<div id="container">
	<button id="discord" class="button" on:click={() => window.open("https://discord.gg/PnQcXP2JjW")}>
		<Fa icon={faDiscord} />
		Discord
	</button>
	<button id="telegram" class="button" on:click={() => window.open("https://t.me/hatcatmc")}>
		<Fa icon={faTelegram} />
		Telegram
	</button>
</div>

<style>
	#container {
		position: fixed;
		width: fit-content;
		bottom: 10px;
		background-color: rgb(0, 0, 0, 0);
		left: 0;
		right: 0;
		margin: auto;
	}

	.button {
		transition: var(--anim-speed);
	}

	#discord {
		margin-right: 10px;
	}

	.button:hover {
		color: black;
	}
	.button {
		background-color: rgb(0, 0, 0, 0);
		border: 0;
		color: white;
		padding: 0;
	}
</style>