<script lang="ts">
	import { appWindow } from '@tauri-apps/api/window'
	appWindow.setResizable(false)
	appWindow.setFocus()
</script>

<div data-tauri-drag-region class="topbar">
	<button class="hide" on:click={() => appWindow.minimize()}>–</button>
	<button class="close" on:click={() => appWindow.close()}>x</button>
</div>

<style>
	.topbar {
		background-color: var(--darker);
		width: inherit;
		height: 30px;
	}

	.close, .hide {
		height: inherit;
		position: fixed;
		display: flexbox;
		text-align: center;
		line-height: 0px;
		border: 0;
		color: var(--light);
		background-color: var(--darker);
		transition: var(--anim-speed);
		width: 30px;
	}

	.hide {
		right: 45px;
	}

	.close {
		right: 10px;
	}

	.hide:hover {
		background-color: white;
		color: var(--darker);
	}

	.close:hover {
		background-color: rgb(200, 0, 0);
		color: white;
	}
</style>