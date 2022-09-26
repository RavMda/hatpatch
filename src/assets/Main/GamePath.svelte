<script lang="ts">
	/*
	 * Font Awesome Icons
	 */
	import Fa from "svelte-fa";
	import {faFolder} from "@fortawesome/free-solid-svg-icons";

	/*
	 * Everything related to stores
	 */
	 import { readyToPatch, gamePath } from "./stores";
	$: isPathValid, readyToPatch.set(isPathValid)

	/*
	 * Everything related to path
	 */

	import { dataDir } from "@tauri-apps/api/path";
	import { open } from "@tauri-apps/api/dialog";
	import { exists } from "@tauri-apps/api/fs";

	function minimizePath(path: string) {
		let length = path.length;

		if (length > 24) {
			path = "..." + path.substring(length - 24, length) ;
		}

		return path
	}
	
	function isMinecraft(path: string) {
		return exists(path + "/versions")
	}

	let realPath = ""
	let displayPath = ""
	let isPathValid = false

	$: {
		displayPath = minimizePath(realPath)

		isMinecraft(realPath).then((exists) => {
			if (typeof(exists) == "boolean") {
				isPathValid = exists
			} else {
				isPathValid = false
			}
		})

		gamePath.set(realPath)
	}
	
	dataDir().then((dir) => {	
		realPath = dir + "\.minecraft"
	})

	async function select() {
		let selected = await open({
			defaultPath: realPath,
			directory: true,
		});

		if (!Array.isArray(selected)) {
			realPath = selected
		}
	}
	
</script>

<p class="title">Game path</p>

<div class="dialog">
	<button type="button" class="dialog-button" on:click={select}>
		<span>{displayPath}</span>
	</button>
	
	<Fa icon={faFolder} color="white" style="display:inline-block; margin-left: 5px;"/>
	
	{#if isPathValid}
		<p class="good">Found the game</p>
	{:else}
		<p class="bad">The path is not valid</p>
	{/if}
</div>

<style>
	.dialog-button {
		margin-top: 15px;
		width: 250px;
		height: 32px;
		border-color: var(--dark);
		background-color: var(--darker);
		font-size: 16px;
		font-weight: 500;
		color: white;
		transition: var(--anim-speed);
		text-overflow:clip;
		border-radius: 8px;
	}

	.dialog-button:hover {
		background-color: var(--light);
	}

	.title {
		font-weight: 700;
		font-size: 20px;
		display: inline;
		color: whitesmoke;
	}

	.good, .bad {
		margin-top: 5px;
	}

	.good {
		color: rgb(128, 255, 128);
	}

	.bad {
		color: rgb(255, 128, 128);
	}
</style>