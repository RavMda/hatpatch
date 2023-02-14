<script lang="ts">
	export let enabled: boolean;
</script>

{#if enabled}
	<div class="heart"><div></div></div>
{/if}

<style>
	.heart {
		margin: auto;
		position: relative;
		width: 5px;
		height: 5px;
		transform: rotate(45deg);
		transform-origin: 20px 20px;
		display: inline;
	}

	.heart div {
		top: 16px;
		left: 16px;
		position: absolute;
		width: 16px;
		height: 16px;
		background: #fff;
		animation: heart 1.2s infinite cubic-bezier(0.215, 0.61, 0.355, 1);
	}
	.heart div:after,
	.heart div:before {
		content: " ";
		position: absolute;
		display: block;
		width: 16px;
		height: 16px;
		background: #fff;
	}
	.heart div:before {
		left: -12px;
		border-radius: 50% 0 0 50%;
	}
	.heart div:after {
		top: -12px;
		border-radius: 50% 50% 0 0;
	}
	@keyframes heart {
		0% {
			transform: scale(0.95);
		}
		5% {
			transform: scale(1.1);
		}
		39% {
			transform: scale(0.85);
		}
		45% {
			transform: scale(1);
		}
		60% {
			transform: scale(0.95);
		}
		100% {
			transform: scale(0.9);
		}
	}
</style>