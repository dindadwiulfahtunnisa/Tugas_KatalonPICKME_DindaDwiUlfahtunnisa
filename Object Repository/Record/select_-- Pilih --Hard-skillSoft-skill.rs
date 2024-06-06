<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>select_-- Pilih --Hard-skillSoft-skill</name>
   <tag></tag>
   <elementGuidId>f21ed317-0dfd-4658-8fbb-064582e7bfbb</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//select[@id='selectSkillCategory--1']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#selectSkillCategory--1</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#selectSkillCategory--1</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>select</value>
      <webElementGuid>82bd7c7e-369f-4f7e-8171-50aacc6beabe</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-collapse</name>
      <type>Main</type>
      <value>#collapseSkill--1</value>
      <webElementGuid>1fdb78b3-eb96-49d4-bb99-ee0ef25c2822</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>selectSkillCategory--1</value>
      <webElementGuid>f4a27333-97f8-40ec-9f59-b89a31b80183</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>custom-select required</value>
      <webElementGuid>5c7050f4-01a1-4ea2-ad7c-cca16572df3a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onchange</name>
      <type>Main</type>
      <value>
                if($(this).val() === ''){
                    $('#txtSkillCategory--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#frame').contents().find('#txtListSkillCategory--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#txtListSkillCategory--1').text($(this).val()).fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                } else {
                    $('#txtSkillCategory--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#frame').contents().find('#txtListSkillCategory--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                    $('#txtListSkillCategory--1').text('('+$(this).find('option:selected').text()+')').fadeTo(100, 0.3, function() { $(this).fadeTo(100, 1.0); })
                }
                autoSave();
                if(finishInitialized){
                    setTimeout(() => {
                        previewSkill({getFromLocal: true}); 
                        isSkillValueBlank()
                    }, 500)
                }
                isSkillValueBlank(); validateRequire($(this))</value>
      <webElementGuid>c5a32b12-8aa7-4127-b4c7-51e79b71860e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                -- Pilih --Hard-skillSoft-skill
              </value>
      <webElementGuid>47b637da-fdbb-4bde-8472-e27055dad3bf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;selectSkillCategory--1&quot;)</value>
      <webElementGuid>a9fad1a5-21eb-49d5-bb3d-2af40e5119b5</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//select[@id='selectSkillCategory--1']</value>
      <webElementGuid>11999def-a4bd-44f7-827b-444da7cdec5a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='collapseSkill--1']/div[2]/div[3]/select</value>
      <webElementGuid>c08e4dda-8cb2-42ec-adf1-7d9d5ef99d1e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Kategori'])[1]/following::select[1]</value>
      <webElementGuid>a7ede4ff-b8b5-4fdd-8f74-2d539c4bb29d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tingkat'])[4]/following::select[2]</value>
      <webElementGuid>15b601a1-e83a-4200-99b1-8552781810e0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Tambah Data'])[3]/preceding::select[1]</value>
      <webElementGuid>8092b848-253a-40ca-b8cc-7e83e4a54ae8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Riwayat Pekerjaan'])[1]/preceding::select[1]</value>
      <webElementGuid>2d0878a8-89ce-4d57-9c62-5933c796df8c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[3]/select</value>
      <webElementGuid>794438bb-bde9-4aec-aba2-060ea05386af</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//select[@id = 'selectSkillCategory--1' and (text() = '
                -- Pilih --Hard-skillSoft-skill
              ' or . = '
                -- Pilih --Hard-skillSoft-skill
              ')]</value>
      <webElementGuid>f52aadcc-42a4-4dbe-ae41-3bb7f3d21944</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
