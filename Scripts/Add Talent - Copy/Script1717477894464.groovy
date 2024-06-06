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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://dev.pickme.metrodataacademy.id/landing-page')

WebUI.click(findTestObject('Object Repository/Record/span_LOGIN'))

WebUI.setText(findTestObject('Object Repository/Record/input_Username_username'), 'mitracamp')

WebUI.setEncryptedText(findTestObject('Object Repository/Record/input_Kata Sandi_password'), 'o+tS4OuGt32s9ezZj287yw==')

WebUI.click(findTestObject('Object Repository/Record/button_LOGIN'))

WebUI.click(findTestObject('Object Repository/Record/a_New Talent'))

WebUI.click(findTestObject('Object Repository/Record/a_Lanjut'))

WebUI.click(findTestObject('Object Repository/Record/div_concat(id(, , body, , )divclass, , intr_e6324d'))

WebUI.click(findTestObject('Object Repository/Record/a_'))

WebUI.setText(findTestObject('Object Repository/Record/input_Nama Lengkap_inputFullName'), 'Dinda dwi ulfahtunnisa')

WebUI.setText(findTestObject('Object Repository/Record/input_Nomor KTP_inputKtp'), '1371064912000007')

WebUI.click(findTestObject('Object Repository/Record/b_Ubah Foto'))

WebUI.click(findTestObject('Object Repository/Record/button_Simpan'))

WebUI.setText(findTestObject('Object Repository/Record/input__inputBirthPlace'), 'Padang')

WebUI.setText(findTestObject('Object Repository/Record/input__inputNationality'), 'Indonesia')

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --                         _17e164'), 
    'Single', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --                         _be629c'), 
    'Female', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --                         _2cfb34'), 
    '1', true)

WebUI.setText(findTestObject('Object Repository/Record/input_No Telepon_inputPhone'), '082268266666')

WebUI.setText(findTestObject('Object Repository/Record/input_E-mail_inputEmail'), 'dinda@gmail.com')

WebUI.setText(findTestObject('Object Repository/Record/input_Alamat_inputProvince'), 'Sumatera Barat')

WebUI.setText(findTestObject('Object Repository/Record/input_Alamat_inputCity'), 'Padang')

WebUI.setText(findTestObject('Object Repository/Record/textarea_Alamat_inputAddress'), 'Pagambiran')

WebUI.click(findTestObject('Object Repository/Record/a_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)'))

WebUI.setText(findTestObject('Object Repository/Record/input_Bahasa_inputLanguageName--1'), 'English')

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --BeginnerIntermediateAdvan_943818'), 
    'Intermediate', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --BeginnerIntermediateAdvan_943818'), 
    'Intermediate', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --BeginnerIntermediateAdvan_943818'), 
    'Intermediate', true)

WebUI.click(findTestObject('Object Repository/Record/i_Simpan Perubahan_fa fa-save'))

WebUI.click(findTestObject('Object Repository/Record/i_(Anda belum mengisi)_fa mt-1 text-primary pl-4'))

WebUI.click(findTestObject('Object Repository/Record/span_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)                   _c26fe3'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)                   _c26fe3'))

WebUI.click(findTestObject('Object Repository/Record/b_(Anda belum mengisi)'))

WebUI.setText(findTestObject('Object Repository/Record/input_Universitas  Sekolah_inputEducationUn_50de64'), 'Universitas Putra Indonesia YPTK')

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --        Junior High Schoo_e0e036'), 
    'Strata 1', true)

WebUI.setText(findTestObject('Object Repository/Record/input_Jurusan_inputEducationMajor--1'), 'Sistem Informasi')

WebUI.setText(findTestObject('Object Repository/Record/input_IPK_inputEducationGpa--1'), '4.00')

WebUI.click(findTestObject('Object Repository/Record/a_Tambah Data_1'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)_1'))

WebUI.setText(findTestObject('Object Repository/Record/input_Skill_inputSkillName--1'), 'SQl')

WebUI.click(findTestObject('Object Repository/Record/input_Skill_inputSkillName--1'))

WebUI.setText(findTestObject('Object Repository/Record/input_Skill_inputSkillName--1'), 'PHP')

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --BasicIntermediateAdvanceExpert'), '1', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --Hard-skillSoft-skill'), '1', true)

WebUI.click(findTestObject('Object Repository/Record/a_Tambah Data_1_2'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)_1_2'))

WebUI.setText(findTestObject('Object Repository/Record/input_Perusahaan_inputCompanyName--1'), 'Semen Padang Hospital')

WebUI.setText(findTestObject('Object Repository/Record/input_Posisi_inputPosition--1'), 'IT Support')

WebUI.selectOptionByValue(findTestObject('Object Repository/Record/select_-- Pilih --                Internshi_9892b6'), 
    'Internship', true)

WebUI.setText(findTestObject('Object Repository/Record/div_Deskripsi Pekerjaan_note-editable'), '<ul><li style="">Jaringan</li></ul>')

WebUI.click(findTestObject('Object Repository/Record/li_Jaringan'))

WebUI.click(findTestObject('Object Repository/Record/span_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/b_(Anda belum mengisi)_1'))

WebUI.setText(findTestObject('Object Repository/Record/input_Nama Project_inputProjectName--1'), 'a')

WebUI.setText(findTestObject('Object Repository/Record/input_Lembaga  Perusahaan_inputProjectSite--1'), 'a')

WebUI.click(findTestObject('Object Repository/Record/li'))

WebUI.setText(findTestObject('Object Repository/Record/div_Deskripsi Project_note-editable'), '<ul><li style="outline: rgb(255, 0, 0) solid 2px;">a</li></ul>')

WebUI.setText(findTestObject('Object Repository/Record/input__inputProjectSkill--1'), 'b')

WebUI.click(findTestObject('Object Repository/Record/span_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/b_(Anda belum mengisi)_1_2'))

WebUI.setText(findTestObject('Object Repository/Record/input_Nama Pelatihan_inputTrainingName--1'), 'Mediatama Web')

WebUI.setText(findTestObject('Object Repository/Record/textarea_Syllabus_inputTrainingSyllabus--1'), '-')

WebUI.click(findTestObject('Object Repository/Record/span_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/b_(Anda belum mengisi)_1_2_3'))

WebUI.setText(findTestObject('Object Repository/Record/input__inputCertificationName--1'), 'aa')

WebUI.setText(findTestObject('Object Repository/Record/input__inputCertificationInstitute--1'), 'aa')

WebUI.click(findTestObject('Object Repository/Record/span_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)_1_2'))

WebUI.setText(findTestObject('Object Repository/Record/input_Nama Organisasi_inputOrganizationName--1'), 'UKM BA FLASH')

WebUI.setText(findTestObject('Object Repository/Record/input_Posisi_inputOrganizationPosition--1'), 'Secretaris')

WebUI.click(findTestObject('Object Repository/Record/span_Tambah Data'))

WebUI.click(findTestObject('Object Repository/Record/div_(Anda belum mengisi)_1'))

WebUI.setText(findTestObject('Object Repository/Record/input__inputExperienceName--1'), 'aa')

WebUI.setText(findTestObject('Object Repository/Record/input__inputExperienceInstitute--1'), 'aa')

WebUI.setText(findTestObject('Object Repository/Record/input__inputPositionName--1'), 'aa')

WebUI.setText(findTestObject('Object Repository/Record/textarea_Deskripsi_inputDescriptionName--1'), 'aa')

WebUI.click(findTestObject('Object Repository/Record/a_Tambah Data_1_2_3'))

WebUI.click(findTestObject('Object Repository/Record/b_(Anda belum mengisi)_1_2_3_4'))

WebUI.setText(findTestObject('Object Repository/Record/input_Nama PenghargaanPrestasi_inputAwardName--1'), 'aa')

WebUI.setText(findTestObject('Object Repository/Record/input_lembaga_inputAwardInstitute--1'), 'aa')

WebUI.click(findTestObject('Object Repository/Record/input_lembaga_inputAwardInstitute--1'))

WebUI.setText(findTestObject('Object Repository/Record/input_Tahun_inputAwardYear--1'), '2019')

WebUI.click(findTestObject('Object Repository/Record/button_Simpan Perubahan'))

WebUI.click(findTestObject('Object Repository/Record/button_Preview CV'))

WebUI.click(findTestObject('Object Repository/Record/button_'))

WebUI.click(findTestObject('Object Repository/Record/button_Download CV'))

WebUI.click(findTestObject('Object Repository/Record/a_PDF'))

WebUI.click(findTestObject('Object Repository/Record/button_Download CV_1'))

WebUI.click(findTestObject('Object Repository/Record/a_Word'))

WebUI.click(findTestObject('Object Repository/Record/button_Preview CV'))

WebUI.click(findTestObject('Object Repository/Record/button_'))

WebUI.switchToWindowTitle('foto pangeran - Penelusuran Google')

WebUI.click(findTestObject('Object Repository/Record/div_Gambar'))

WebUI.setText(findTestObject('Object Repository/Record/textarea_foto pangeran'), 'foto pangeran kartun')

WebUI.rightClick(findTestObject('Object Repository/Record/img_Siapa nama pangeran yang meminang Cinde_c7d45f'))

WebUI.switchToWindowTitle('App / CV-ME')

WebUI.click(findTestObject('Object Repository/Record/b_Hapus Foto'))

WebUI.click(findTestObject('Object Repository/Record/b_Ya'))

WebUI.click(findTestObject('Object Repository/Record/b_Ubah Foto'))

WebUI.click(findTestObject('Object Repository/Record/button_Simpan'))

WebUI.click(findTestObject('Object Repository/Record/div_Tempat Lahir'))

WebUI.setText(findTestObject('Object Repository/Record/input__inputBirthPlace'), 'Sawahlunto')

WebUI.setText(findTestObject('Object Repository/Record/input_Alamat_inputCity'), 'Padang')

WebUI.click(findTestObject('Object Repository/Record/div_Alamat_form-group col-md-6 col-6'))

WebUI.setText(findTestObject('Object Repository/Record/input_Alamat_inputCity'), 'Sawahlunto')

WebUI.click(findTestObject('Object Repository/Record/i_Tingkat_text-bottom fa fa-trash'))

WebUI.click(findTestObject('Object Repository/Record/button_Batal'))

WebUI.click(findTestObject('Object Repository/Record/i_Issues_text-bottom fa fa-trash'))

WebUI.click(findTestObject('Object Repository/Record/b_Ya'))

WebUI.click(findTestObject('Object Repository/Record/button_Simpan Perubahan_1'))

WebUI.closeBrowser()

