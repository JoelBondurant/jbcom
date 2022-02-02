const photos = (() => {

	let numPhotos = 0;
	let gridId = 'NULL';
	let photoIndex = 0;
	let touchStartX = 0;
	let touchEndX = 0;

	function bind(uid) {
		gridId = uid;
	}

	function setNumber(n) {
		numPhotos = n;
	}

	function injectPhotos() {
		const photoGrid = document.getElementById(gridId);
		photoGrid.innerHTML = '';
		photoGrid.setAttribute('style', 'display: grid; grid-template-columns: repeat(auto-fill, 212px);');
		for (n = 1; n <= numPhotos; n++) {
			const nstr = n.toString().padStart(3, '0');
			const ele = `
				<div>
					<button onclick="photos.injectPhoto(${n-1});">
						<img src="photos/thumbnails/thumbnail_${nstr}.jpg">
					</button>
				</div>`;
			photoGrid.innerHTML += ele;
		}
	}

	function injectPhoto(indx) {
		photoIndex = indx;
		const photoGrid = document.getElementById(gridId);
		photoGrid.innerHTML = '';
		photoGrid.setAttribute('style', 'display: grid; grid-template-columns: 1fr auto 1fr;');
		const nstr = (indx + 1).toString().padStart(3, '0');
		const ele = `
			<div onclick="photos.navigateLeft();" style="width: 100%;"></div>
			<div style="text-align: center;">
				<button onclick="photos.navigateRight();">
					<img src="photos/photo_${nstr}.jpg" style="max-width: 95vw; max-height: 95vh;">
				</button>
			</div>
			<div onclick="photos.injectPhotos();"></div>`;
		photoGrid.innerHTML = ele;
	}

	function navigateRight() {
		photoIndex += 1;
		if (photoIndex >= numPhotos) {
			photoIndex = 0;
		}
		injectPhoto(photoIndex);
	}

	function navigateLeft() {
		photoIndex -= 1;
		if (photoIndex < 0) {
			photoIndex = numPhotos - 1;
		}
		injectPhoto(photoIndex);
	}

	function addNavigationListeners() {
		document.addEventListener('keydown', (evt) => {
			if (evt.shiftKey && evt.altKey && (evt.key == 'ArrowRight')) {
				navigateRight();
			}
			else if (evt.shiftKey && evt.altKey && (evt.key == 'ArrowLeft')) {
				navigateLeft();
			}
			else if (evt.shiftKey && evt.altKey && (evt.key == 'ArrowUp')) {
				injectPhoto(photoIndex);
			}
			else if (evt.shiftKey && evt.altKey && (evt.key == 'ArrowDown')) {
				injectPhotos();
			}
		});
		document.addEventListener('touchstart', (evt) => {
			touchStartX = evt.changedTouches[0].screenX;
		});
		document.addEventListener('touchend', (evt) => {
			touchEndX = evt.changedTouches[0].screenX;
			const gap = 100;
			if (touchEndX < touchStartX - gap) {
				navigateRight();
			} else if (touchEndX > touchStartX + gap) {
				navigateLeft();
			}
		});
	}

	function install(uid, n) {
		bind(uid);
		setNumber(n);
		injectPhotos();
		addNavigationListeners();
	}

	return {
		'bind': bind,
		'setNumber': setNumber,
		'injectPhoto': injectPhoto,
		'injectPhotos': injectPhotos,
		'navigateRight': navigateRight,
		'navigateLeft': navigateLeft,
		'addNavigationListeners': addNavigationListeners,
		'install': install
	}
})();
