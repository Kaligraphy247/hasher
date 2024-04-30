<script lang="ts">
	import '../app.css';
	// Supports weights 100-900
	import '@fontsource-variable/dm-sans';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { emit, listen } from '@tauri-apps/api/event';

	type HashText = {
		text: string;
		function_type: string;
	};

	// Initialize variables for state management
	let count: number;
	let textToHash: string = '';
	let result: string = '';
	let autoUpdateHash: boolean = false;
	let hashingAlgo: string = 'sha2_256';

	onMount(async () => {
		count = await invoke('get_count');
		console.log(count);
	});

	async function incrementCount() {
		count = await invoke('increment_count');
		console.log(count);
	}

	async function decrementCount() {
		count = await invoke('decrement_count');
		console.log(count);
	}

	// IPC Event Listener
	listen('hash_result', (event) => {
		let payload: HashText = event.payload as HashText;
		result = payload.text;
		// = event.payload.result;
		console.log(event);
	});

	/** Send Text to Rust and return result to result variable */
	async function hashText() {
		result = await invoke(hashingAlgo, { data: textToHash });
	}

	/** Automatically update result when text is changed */
	async function autoHashText() {
		emit('hash_text', {
			text: textToHash,
			function_type: hashingAlgo
		});
	}
</script>

<body class="bg-zinc-900 min-h-screen min-w-full text-white text-lg p-4">
	<!-- <slot /> -->
	<h2 class="text-2xl font-bold py-4 text-center">Hasher</h2>
	<h3 class="text-xl font-semibold mb-2 text-center">Try out different hashing algorithms!</h3>

	<div class="flex space-x-4">
		<p>Count</p>
		<button class="border px-2 py-0.5 rounded" on:click={incrementCount}>+</button>
		<p>{count}</p>
		<button class="border px-2 py-0.5 rounded" on:click={decrementCount}>-</button>
	</div>

	<div class="flex border-0 rounded-md shadow gap-8 h-[460px] max-h-[460px]">
		<section class="border w-2/3 rounded-md px-4 py-2 border-zinc-700">
			<p>Selected Algorithm:&ensp;<span class="font-semibold">{hashingAlgo}</span></p>
			<div class="mt-2">
				<label>
					<span>Text to hash</span>
					<input
						class="bg-zinc-950 px-2 py-1 rounded-md text-white placeholder:text-sm"
						type="text"
						bind:value={textToHash}
						placeholder="Enter text"
						on:input={() => (autoUpdateHash ? autoHashText() : null)}
					/>
				</label>
				<button class="px-2 py-1 bg-zinc-800 rounded-md text-white" on:click={hashText}>Hash</button
				>

				<div class="flex justify-between w-full mt-8">
					<p class="my-auto">Result</p>
					<div class="my-auto flex text-sm">
						<p class="my-auto">Auto Hash</p>
						<input
							class="bg-zinc-950 m-2 rounded-md text-white w-5 h-5"
							type="checkbox"
							bind:checked={autoUpdateHash}
							name="autoHash"
							id="autoHash"
						/>
					</div>
				</div>
				<textarea
					class="bg-zinc-950 px-2 py-1 rounded-md text-white font-mono min-h-[290px] h-full w-full resize-none placeholder:text-sm"
					readonly
					placeholder="Result will appear here"
					>{textToHash !== '' ? result : (result = '')}</textarea
				>
			</div>
		</section>
		<aside class="border w-1/3 rounded-md px-4 py-2 border-zinc-700">
			<h3 class="text-lg font-medium">Hashing Algorithm</h3>
			<div>
				<label for="sha2_256"
					>SHA2-256
					<input type="radio" value="sha2_256" bind:group={hashingAlgo} />
				</label>
				<br />
				<label for="sha3_256"
					>SHA3-256
					<input type="radio" value="sha3_256" bind:group={hashingAlgo} />
				</label>
			</div>
		</aside>
	</div>
</body>
