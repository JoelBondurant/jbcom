from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates


app = FastAPI()

app.mount('/css', StaticFiles(directory='css'), name='css')
app.mount('/js', StaticFiles(directory='js'), name='js')
app.mount('/img', StaticFiles(directory='img'), name='img')
app.mount('/photos', StaticFiles(directory='photos'), name='photos')

templates = Jinja2Templates(directory='html')

@app.get('/', response_class=HTMLResponse)
async def get_index(req: Request):
	return templates.TemplateResponse('index.html', {'request': req})

@app.get('/resume', response_class=HTMLResponse)
async def get_resume(req: Request):
	return templates.TemplateResponse('resume.html', {'request': req})

@app.get('/photos', response_class=HTMLResponse)
async def get_photos(req: Request):
	return templates.TemplateResponse('photos.html', {'request': req})

@app.get('/notes', response_class=HTMLResponse)
async def get_notes(req: Request):
	return templates.TemplateResponse('notes.html', {'request': req})

@app.get('/calendly', response_class=HTMLResponse)
async def get_calendly(req: Request):
	return templates.TemplateResponse('calendly.html', {'request': req})

