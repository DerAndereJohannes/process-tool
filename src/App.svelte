
<script lang="ts">
  import EntitySpace from './lib/EntitySpace.svelte'
  import WorkSpace from './lib/WorkSpace.svelte'
  import { listen, Event as TauriEvent } from '@tauri-apps/api/event'

    const isOpenClass = 'modal-is-open';
    const openingClass = 'modal-is-opening';
    const closingClass = 'modal-is-closing';
    const animationDuration = 50; // ms
    let progressModal: HTMLDialogElement;
    let progressModalText: HTMLOListElement;
    let progressModalBar: HTMLProgressElement;

    // Open modal
const openModal = (modal: HTMLDialogElement) => {
  if (isScrollbarVisible()) {
    document.documentElement.style.setProperty('--scrollbar-width', `${getScrollbarWidth()}px`);
  }
  document.documentElement.classList.add(isOpenClass, openingClass);
  setTimeout(() => {
    document.documentElement.classList.remove(openingClass);
  }, animationDuration);
  modal.setAttribute('open', "true");
}

// Close modal
const closeModal = (modal: HTMLDialogElement) => {
  document.documentElement.classList.add(closingClass);
  setTimeout(() => {
    document.documentElement.classList.remove(closingClass, isOpenClass);
    document.documentElement.style.removeProperty('--scrollbar-width');
    modal.removeAttribute('open');
  }, animationDuration);
}

// Close with Esc key
document.addEventListener('keydown', event => {
  if (event.key === 'Escape') {
    if (progressModal.getAttribute('open') == "true") {
        closeModal(progressModal);
    } else {
        openModal(progressModal);
    }
  }
});


// Get scrollbar width
const getScrollbarWidth = () => {

  // Creating invisible container
  const outer = document.createElement('div');
  outer.style.visibility = 'hidden';
  outer.style.overflow = 'scroll'; // forcing scrollbar to appear
  document.body.appendChild(outer);

  // Creating inner element and placing it in the container
  const inner = document.createElement('div');
  outer.appendChild(inner);

  // Calculating difference between container's full width and the child width
  const scrollbarWidth = (outer.offsetWidth - inner.offsetWidth);

  // Removing temporary elements from the DOM
  outer.parentNode.removeChild(outer);

  return scrollbarWidth;
}

// Is scrollbar visible
const isScrollbarVisible = () => {
  return document.body.scrollHeight > screen.height;
}

  listen("progress", function (evt: TauriEvent<any>) {
    if (evt.payload.current_step == 1) {
        openModal(progressModal);
        progressModalText.innerHTML = "";
        progressModalBar.max = evt.payload.total_steps;
    }
    progressModalText.innerHTML += "<li>" + evt.payload.current_task + "</li>";
    progressModalBar.value = evt.payload.current_step;
  });
</script>


<div class="main-screen">
    <!-- Entity Selection -->
    <EntitySpace />

    <!-- Tabbed Selection -- 1. View Object -- 2. Programs -- -->
    <WorkSpace />
</div>


<dialog id="progress-modal" bind:this={progressModal}>
  <article class="progress-card">
    <h3>Executing Plugin</h3>
        <ol bind:this={progressModalText}></ol>
    <progress id="progress-bar" value="0" max="100" bind:this={progressModalBar}></progress>
    <footer>
      <a href="#confirm" role="button" on:click="{() =>closeModal(progressModal)}">Close</a>
    </footer>
  </article>
</dialog>


<style>
.main-screen {
    display: grid;
    grid-template-columns: fit-content(33%) auto;
    grid-column-gap: 20px;
    grid-row-gap: 20px;
    justify-items: stretch;
    align-items: stretch;
    height: 100%;
    width: 100%;
}

.progress-card {
    min-width: 600px;
}
</style>
