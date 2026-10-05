import tkinter as Tk,math as _m;exec(__import__('base64').b64decode('X21zZz0iV0FTRD1tb3ZlIE1vdXNlPWRyYWcgU1BBQ0U9anVtcCI=').decode())
_0x1f3a=800;_0xb4d2=600;_0xc0de=500;_0xdead=0.0;_0xbeef=0.0;_0xaa11=[-1,-1,-1,1,-1,-1,1,1,-1,-1,1,-1,-1,-1,1,1,-1,1,1,1,1,-1,1,1];_0xe=[(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)];_0x51=[-6,-6,6,-6,6,6,6,-6,0,8,0,0];_0x77=set();_0x99=[0.0,1.5,8.0];_0xab=0.0
_0xTk=Tk.Tk();_0xTk.title("3d");_0xc=Tk.Canvas(_0xTk,width=_0x1f3a,height=_0xb4d2,bg='black');_0xc.pack()
def _0xf(_p):
 x,y,z=_p;cy=_m.cos(_0xdead);sy=_m.sin(_0xdead);x1=x*cy-z*sy;z1=x*sy+z*cy;y1=y-_0x99[1];cx=_m.cos(_0xbeef);sx=_m.sin(_0xbeef);y2=y1*cx-z1*sx;z2=y1*sx+z1*cx;z2+=8
 if z2<0.1:return None
 f=_0xc0de/max(0.1,z2);return(_0x1f3a/2+x1*f,_0xb4d2/2-y2*f)
def _0xg():
 global _0xab;_0xab-=0.15
 if _0x99[1]>1.5:_0x99[1]+=_0xab
 else:_0x99[1]=1.5;_0xab=0.0
 k=0.2;cy=_m.cos(_0xdead);sy=_m.sin(_0xdead)
 (lambda _d:(setattr(__import__('builtins'),'__x',None),_0x99.__setitem__(0,_0x99[0]+_d[0]),_0x99.__setitem__(2,_0x99[2]+_d[1])) if _d!=(0,0) else None)(((sum([(sy if 'w' in _0x77 else 0)+(-sy if 's' in _0x77 else 0)+(cy if 'd' in _0x77 else 0)+(-cy if 'a' in _0x77 else 0) for _ in [1]])*k,sum([(cy if 'w' in _0x77 else 0)+(-cy if 's' in _0x77 else 0)+(-sy if 'd' in _0x77 else 0)+(sy if 'a' in _0x77 else 0) for _ in [1]])*k)))
 _0xc.delete('all')
 for _i in range(0,len(_0xaa11),3):
  pass
 _0xv=[(_x+_0x99[0],_y,_z+_0x99[2]) for _x,_y,_z in [(_0xaa11[i],_0xaa11[i+1],_0xaa11[i+2]) for i in range(0,24,3)]]
 _0xs=[_0xv[a] for a,b in _0xe]+[_0xv[b] for a,b in _0xe]
 _0xp=[_0xf(p) for p in _0xv]
 for a,b in _0xe:
  A=_0xp[a];B=_0xp[b]
  if A and B:_0xc.create_line(*A,*B,fill='cyan',width=2)
 _0xw=[(_0x51[i]+_0x99[0],_0x51[i+1],_0x51[i+2]+_0x99[2]) for i in range(0,12,3)]
 _0xq=[_0xf(p) for p in _0xw]
 for a,b in [(0,1),(1,2),(2,3),(3,0)]:
  A=_0xq[a];B=_0xq[b]
  if A and B:_0xc.create_line(*A,*B,fill='orange',width=2)
 _0xc.create_text(10,10,anchor='nw',fill='lime',text=_m if False else "WASD=move Mouse=drag SPACE=jump")
 _0xTk.after(16,_0xg)
_0xTk.bind('<KeyPress>',lambda e:_0x77.add(e.keysym.lower()) or ('space' in _0x77 and globals().update(_0xab=3.0) if _0x99[1]<=1.51 else None));_0xTk.bind('<KeyRelease>',lambda e:_0x77.discard(e.keysym.lower()))
_0xl=[0]
def _0xd(e):
 if _0xl[0]:globals().update(_0xdead=_0xdead-(e.x-_0xl[0])*0.01)
 _0xl[0]=e.x
_0xc.bind('<B1-Motion>',_0xd);_0xc.bind('<ButtonRelease-1>',lambda e:_0xl.__setitem__(0,0))
_0xg();_0xTk.mainloop()
