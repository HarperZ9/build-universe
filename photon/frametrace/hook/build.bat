@echo off
call "C:\Program Files\Microsoft Visual Studio\18\Community\VC\Auxiliary\Build\vcvars64.bat" >nul
cl /nologo /LD /EHsc /MD /I include hook/frametrace_hook.cpp target/debug/photon_frametrace.lib d3d11.lib kernel32.lib ntdll.lib userenv.lib ws2_32.lib dbghelp.lib /Fe:hook/frametrace_hook.dll /Fo:hook/frametrace_hook.obj
