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

//Logout
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

WebUI.click(findTestObject('Object Repository/Record/a_Talent'))

WebUI.click(findTestObject('Object Repository/Record/a_Lowongan'))

WebUI.click(findTestObject('Object Repository/Record/a_mitracamp (1)'))

WebUI.takeScreenshot()

WebUI.click(findTestObject('Object Repository/Record/a_Logout'))

WebUI.delay(GlobalVariable.Waiting)

WebUI.closeBrowser()
