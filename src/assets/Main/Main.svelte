<script lang="ts">
	import GamePath from "./GamePath.svelte";
    import { readyToPatch, gamePath } from "./stores";
	import { listen } from "@tauri-apps/api/event"

	let output = "-"
	let outputColor = "grey"

	interface Output {
		message: string,
		message_type: Type
	}

	enum Type {
		INFO = "INFO",
		ERROR = "ERROR",
		SUCCESS = "SUCCESS"
	}
	
	const _ = listen("output", (event) => {
		if (typeof(event.payload) === "object") {
			let payload = event.payload as Output
			output = payload.message

			outputColor = "rgb(85, 85, 85)"

			setTimeout(() => {
				switch(payload.message_type) {
					case Type.INFO:
						outputColor = "grey"
						break
					case Type.ERROR:
						outputColor = "rgb(255, 130, 130)"
						break
					case Type.SUCCESS:
						outputColor = "rgb(160, 255, 160)"
						break
				}
			}, 200)

		}
	})

	let isReady = false

	readyToPatch.subscribe((value) => {
		isReady = value
	})

	readyToPatch.set(false)

	import { invoke } from "@tauri-apps/api/tauri";

	function patch() {
		invoke("patch_game", {gamePath: $gamePath})
		readyToPatch.set(false)
	}
</script>

<div class="main">
	<GamePath/>
	<p class="info" style="visibility: {!isReady && "visible" || "visible"}; color: {outputColor};'">
		{output}
	</p>
	<button class="patch {isReady && "pulsating"}" disabled={!isReady} on:click={patch}>
		Start
	</button>
</div>

<style>
	.main {
		display: block;
		margin: auto;
		text-align: center;
		margin-top: 25px;
	}

	.info {
		margin-top: 80px;
		transition: var(--anim-speed);
	}

	.patch {
		width: 180px;
		height: 40px;
		margin-top: 5px;
		font-weight: 600;
		color: white;
		background-color: var(--darker);
		border-radius: 6px;
		border-width: 0;
		transition: 0.3s;
	}

	.patch:enabled:hover {
		box-shadow: 0 0 20px 1px white !important;
		color: black;
		background-color: white;
	}

	.patch:disabled {
		color: grey;
		cursor: not-allowed;
	}

	.patch:disabled:hover{
		box-shadow: none;
		color: grey;
		background-color: var(--darker);
	}

	.disabled {
		background-color: var(--darker);
		color: white;
	}

	.pulsating {
		animation: pulsate;
		animation-iteration-count: infinite;
		animation-duration: 1s;
	}

	@keyframes pulsate {
		0% {
			box-shadow: 0 0 0 0 rgba(255, 255, 255, 0.4);
		}
		70% {
			box-shadow: 0 0 0 10px rgba(255, 255, 255, 0);
		}
		100% {
			box-shadow: 0 0 0 0 rgba(255, 255, 255, 0);
		}
	}

</style>