import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//Lowongan

'Membuka Web Browser'
WebUI.openBrowser('')

WebUI.maximizeWindow()

'Menampilkan Halaman Web PICKME'
WebUI.navigateToUrl(GlobalVariable.URL)

WebUI.takeScreenshot()

'Klik Menu Login'
WebUI.click(findTestObject('Object Repository/Record/span_LOGIN'))

'Input Username dan Password'
WebUI.setText(findTestObject('Object Repository/Record/input_Username_username'), GlobalVariable.DataUsername)

WebUI.setText(findTestObject('Object Repository/Record/input_Kata Sandi_password'), GlobalVariable.DataPassword)

'Klik Button Login'
WebUI.click(findTestObject('Object Repository/Record/button_LOGIN'))

WebUI.takeScreenshot()

'Klik Menu Lowongan'
WebUI.click(findTestObject('Object Repository/Lowongan/a_Lowongan'))

WebUI.setText(findTestObject('Object Repository/Lowongan/input_Logout_searchTitle'), 'quality assurance')

WebUI.click(findTestObject('Object Repository/Lowongan/p_004 - Quality Assurance'))

WebUI.click(findTestObject('Object Repository/Lowongan/a_Detail'))

'Memilih Talent Yang Terdsedia'
WebUI.click(findTestObject('Object Repository/Lowongan/button_Select Talent'))

WebUI.click(findTestObject('Object Repository/Lowongan/button_Pilih'))

WebUI.click(findTestObject('Object Repository/Lowongan/button_Close'))

WebUI.click(findTestObject('Object Repository/Lowongan/span_'))

WebUI.click(findTestObject('Object Repository/Lowongan/div_Talent Details                         _c6ae76'))

WebUI.click(findTestObject('Object Repository/Lowongan/button_Apply'))

WebUI.click(findTestObject('Object Repository/Lowongan/button_OK'))

WebUI.click(findTestObject('Object Repository/Lowongan/span_Cari berdasarkan posisi'))

WebUI.click(findTestObject('Object Repository/Lowongan/input_Terakhir diupdate_timeInterval'))

WebUI.click(findTestObject('Object Repository/Lowongan/input_Terakhir diupdate_timeInterval_1'))

WebUI.click(findTestObject('Object Repository/Lowongan/input_Terakhir diupdate_timeInterval_1_2'))

WebUI.click(findTestObject('Object Repository/Lowongan/input_Kapanpun_timeInterval'))

WebUI.delay(GlobalVariable.Waiting)
WebUI.closeBrowser()

