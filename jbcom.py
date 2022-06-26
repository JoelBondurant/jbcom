import aioredis
import orjson
from fastapi import FastAPI, Request, Header
from fastapi.responses import HTMLResponse, ORJSONResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates


app_context = {}

app = FastAPI()
app_context['app'] = app

tn = 'share'
app.mount(f'/{tn}', StaticFiles(directory=tn), name=tn)
app.mount('/css', StaticFiles(directory='css'), name='css')
app.mount('/js', StaticFiles(directory='js'), name='js')
app.mount('/img', StaticFiles(directory='img'), name='img')
app.mount('/photos', StaticFiles(directory='photos'), name='photos')

templates = Jinja2Templates(directory='html')


@app.on_event('startup')
async def on_startup():
	with open('/secrets/jbcom.json', 'r') as fin:
		config = orjson.loads(fin.read())
	app_context['config'] = config

	redis = await aioredis.create_redis_pool(
		'redis://' + config['redis']['host'],
		password=config['redis']['password'])
	app_context['redis'] = redis


@app.get('/', response_class=HTMLResponse)
async def get_index(req: Request):
	hit_counter = await app_context['redis'].incr('jbcom_hit_counter')
	return templates.TemplateResponse('index.html', {
		'request': req,
		'hit_counter': hit_counter
	})

@app.get('/resume', response_class=HTMLResponse, include_in_schema=False)
async def get_resume(req: Request):
	return templates.TemplateResponse('resume.html', {'request': req})

@app.get('/photos', response_class=HTMLResponse)
async def get_photos(req: Request):
	return templates.TemplateResponse('photos.html', {'request': req})

@app.get('/notes', response_class=HTMLResponse)
async def get_notes(req: Request):
	return templates.TemplateResponse('notes.html', {'request': req})

@app.get('/calendly', response_class=HTMLResponse, include_in_schema=False)
async def get_calendly(req: Request):
	return templates.TemplateResponse('calendly.html', {'request': req})

@app.get('/blog', response_class=HTMLResponse)
async def get_blog(req: Request):
	return templates.TemplateResponse('blog.html', {'request': req})

@app.get('/trace', response_class=ORJSONResponse)
async def get_trace(req: Request):
	resp = {
		'X-Forwarded-For': req.headers.get('X-Forwarded-For'),
		'User-Agent': req.headers.get('User-Agent'),
	}
	return resp

