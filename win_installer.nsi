;
; Windows Installer Script generated by the HM NIS Edit Script Wizard.
;
; You can get required libs from https://ipfs.frsqr.xyz/ipfs/bafybeibdyouh4ueqmwm6uwrpnwm7wkekngm477ubokkqxijeqzz7gnzptu/libs.zip
; and put them to ./libs/ folder.
;

; HM NIS Edit Wizard helper defines
!define PRODUCT_NAME "FireLaunch"
!define PRODUCT_VERSION "0.1.0"
!define PRODUCT_PUBLISHER "Egor Ternovoy"
!define PRODUCT_WEB_SITE "https://frsqr.xyz/"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\firesquare-launcher.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
!define PRODUCT_UNINST_ROOT_KEY "HKLM"

; MUI 1.67 compatible ------
!include "MUI.nsh"

; MUI Settings
!define MUI_ABORTWARNING
!define MUI_ICON "resources\favicon.ico"
!define MUI_UNICON "resources\deinst.ico"

; Language Selection Dialog Settings
!define MUI_LANGDLL_REGISTRY_ROOT "${PRODUCT_UNINST_ROOT_KEY}"
!define MUI_LANGDLL_REGISTRY_KEY "${PRODUCT_UNINST_KEY}"
!define MUI_LANGDLL_REGISTRY_VALUENAME "NSIS:Language"

; Welcome page
!insertmacro MUI_PAGE_WELCOME
; License page
!insertmacro MUI_PAGE_LICENSE "LICENSE"
; Directory page
!insertmacro MUI_PAGE_DIRECTORY
; Instfiles page
!insertmacro MUI_PAGE_INSTFILES
; Finish page
!define MUI_FINISHPAGE_RUN "$INSTDIR\firesquare-launcher.exe"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_INSTFILES

; Language files
!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "Russian"
!insertmacro MUI_LANGUAGE "Ukrainian"

; MUI end ------

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "${PRODUCT_NAME}-${PRODUCT_VERSION}-install.exe"
InstallDir "$PROGRAMFILES\FireLaucnh"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

Function .onInit
  !insertmacro MUI_LANGDLL_DISPLAY
FunctionEnd

Section "MainSection" SEC01
  SetOutPath "$INSTDIR"
  SetOverwrite try
  File "libs\libadwaita-1-0.dll"
  File "libs\libbrotlicommon.dll"
  File "libs\libbrotlidec.dll"
  File "libs\libbz2-1.dll"
  File "libs\libcairo-2.dll"
  File "libs\libcairo-gobject-2.dll"
  File "libs\libcairo-script-interpreter-2.dll"
  File "libs\libdatrie-1.dll"
  File "libs\libdeflate.dll"
  File "libs\libepoxy-0.dll"
  File "libs\libexpat-1.dll"
  File "libs\libffi-8.dll"
  File "libs\libfontconfig-1.dll"
  File "libs\libfreetype-6.dll"
  File "libs\libfribidi-0.dll"
  File "libs\libgcc_s_seh-1.dll"
  File "libs\libgdk_pixbuf-2.0-0.dll"
  File "libs\libgio-2.0-0.dll"
  File "libs\libglib-2.0-0.dll"
  File "libs\libgmodule-2.0-0.dll"
  File "libs\libgobject-2.0-0.dll"
  File "libs\libgraphene-1.0-0.dll"
  File "libs\libgraphite2.dll"
  File "libs\libgtk-4-1.dll"
  File "libs\libharfbuzz-0.dll"
  File "libs\libiconv-2.dll"
  File "libs\libintl-8.dll"
  File "libs\libjbig-0.dll"
  File "libs\libjpeg-8.dll"
  File "libs\libLerc.dll"
  File "libs\liblzma-5.dll"
  File "libs\liblzo2-2.dll"
  File "libs\libpango-1.0-0.dll"
  File "libs\libpangocairo-1.0-0.dll"
  File "libs\libpangoft2-1.0-0.dll"
  File "libs\libpangowin32-1.0-0.dll"
  File "libs\libpcre2-8-0.dll"
  File "libs\libpixman-1-0.dll"
  File "libs\libpng16-16.dll"
  File "libs\libstdc++-6.dll"
  File "libs\libthai-0.dll"
  File "libs\libtiff-6.dll"
  File "libs\libwebp-7.dll"
  File "libs\libwinpthread-1.dll"
  File "libs\libzstd.dll"
  File "libs\zlib1.dll"
  SetOverwrite ifnewer
  File "target\release\firesquare-launcher.exe"
  CreateDirectory "$SMPROGRAMS\FireLaucnh"
  CreateShortCut "$SMPROGRAMS\FireLaucnh\FireLaucnh.lnk" "$INSTDIR\firesquare-launcher.exe"
  CreateShortCut "$DESKTOP\FireLaucnh.lnk" "$INSTDIR\firesquare-launcher.exe"
SectionEnd

Section -Post
  WriteUninstaller "$INSTDIR\uninst.exe"
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\firesquare-launcher.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\firesquare-launcher.exe"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
SectionEnd


Function un.onUninstSuccess
  HideWindow
  MessageBox MB_ICONINFORMATION|MB_OK "�������� ��������� ${PRODUCT_NAME} ���� ������� ���������."
FunctionEnd

Function un.onInit
!insertmacro MUI_UNGETLANGUAGE
  MessageBox MB_ICONQUESTION|MB_YESNO|MB_DEFBUTTON2 "�� ������� � ���, ��� ������� ������� ${PRODUCT_NAME} � ��� ���������� ���������?" IDYES +2
  Abort
FunctionEnd

Section Uninstall
  Delete "$INSTDIR\uninst.exe"
  Delete "$INSTDIR\firesquare-launcher.exe"
  Delete "$INSTDIR\zlib1.dll"
  Delete "$INSTDIR\libzstd.dll"
  Delete "$INSTDIR\libwinpthread-1.dll"
  Delete "$INSTDIR\libwebp-7.dll"
  Delete "$INSTDIR\libtiff-6.dll"
  Delete "$INSTDIR\libthai-0.dll"
  Delete "$INSTDIR\libstdc++-6.dll"
  Delete "$INSTDIR\libpng16-16.dll"
  Delete "$INSTDIR\libpixman-1-0.dll"
  Delete "$INSTDIR\libpcre2-8-0.dll"
  Delete "$INSTDIR\libpangowin32-1.0-0.dll"
  Delete "$INSTDIR\libpangoft2-1.0-0.dll"
  Delete "$INSTDIR\libpangocairo-1.0-0.dll"
  Delete "$INSTDIR\libpango-1.0-0.dll"
  Delete "$INSTDIR\liblzo2-2.dll"
  Delete "$INSTDIR\liblzma-5.dll"
  Delete "$INSTDIR\libLerc.dll"
  Delete "$INSTDIR\libjpeg-8.dll"
  Delete "$INSTDIR\libjbig-0.dll"
  Delete "$INSTDIR\libintl-8.dll"
  Delete "$INSTDIR\libiconv-2.dll"
  Delete "$INSTDIR\libharfbuzz-0.dll"
  Delete "$INSTDIR\libgtk-4-1.dll"
  Delete "$INSTDIR\libgraphite2.dll"
  Delete "$INSTDIR\libgraphene-1.0-0.dll"
  Delete "$INSTDIR\libgobject-2.0-0.dll"
  Delete "$INSTDIR\libgmodule-2.0-0.dll"
  Delete "$INSTDIR\libglib-2.0-0.dll"
  Delete "$INSTDIR\libgio-2.0-0.dll"
  Delete "$INSTDIR\libgdk_pixbuf-2.0-0.dll"
  Delete "$INSTDIR\libgcc_s_seh-1.dll"
  Delete "$INSTDIR\libfribidi-0.dll"
  Delete "$INSTDIR\libfreetype-6.dll"
  Delete "$INSTDIR\libfontconfig-1.dll"
  Delete "$INSTDIR\libffi-8.dll"
  Delete "$INSTDIR\libexpat-1.dll"
  Delete "$INSTDIR\libepoxy-0.dll"
  Delete "$INSTDIR\libdeflate.dll"
  Delete "$INSTDIR\libdatrie-1.dll"
  Delete "$INSTDIR\libcairo-script-interpreter-2.dll"
  Delete "$INSTDIR\libcairo-gobject-2.dll"
  Delete "$INSTDIR\libcairo-2.dll"
  Delete "$INSTDIR\libbz2-1.dll"
  Delete "$INSTDIR\libbrotlidec.dll"
  Delete "$INSTDIR\libbrotlicommon.dll"
  Delete "$INSTDIR\libadwaita-1-0.dll"

  Delete "$DESKTOP\FireLaucnh.lnk"
  Delete "$SMPROGRAMS\FireLaucnh\FireLaucnh.lnk"

  RMDir "$SMPROGRAMS\FireLaucnh"
  RMDir "$INSTDIR"

  DeleteRegKey ${PRODUCT_UNINST_ROOT_KEY} "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
  SetAutoClose true
SectionEnd
