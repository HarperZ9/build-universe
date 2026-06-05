@echo off
call "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat" >nul
cl /nologo /MD /I include c_demo/demo.c target/debug/photon_frametrace.lib kernel32.lib ntdll.lib userenv.lib ws2_32.lib dbghelp.lib /Fe:c_demo/demo.exe /Fo:c_demo/demo.obj
