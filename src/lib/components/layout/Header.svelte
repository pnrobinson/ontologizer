<script>
    import { createEventDispatcher } from 'svelte';
    // import FullScreen from './header/FullScreen.svelte';
    import Translate from './header/Translate.svelte';
    import ThemeSwitch from './header/ThemeSwitch.svelte';

    const dispatch = createEventDispatcher();

    function toggleSidebar() {
        dispatch('toggleSidebar');
    }
</script>

<header
    class="h-16 bg-white border-b border-gray-200 shadow-sm sticky top-0 flex"
>
    <!-- Use Menu Icon to open the sidenav -->
    <div class="flex-none">
        <button
            aria-label="open logs"
            class="flex items-center m-3 p-2 text-gray-600 hover:bg-gray-200 drounded-full active:bg-gray-300"
            on:click={toggleSidebar}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="icon icon-tabler icon-tabler-archive"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                stroke-width="2"
                stroke="currentColor"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                />
            </svg>
        </button>
    </div>
    <div class="flex border-0 grow flex-row-reverse">
        <div class="flex-none">
            <ThemeSwitch />
        </div>
        <!-- <div class="flex-none">
			<FullScreen />
		</div> -->
        <div class="flex-none">
            <Translate />
        </div>
    </div>
</header>
